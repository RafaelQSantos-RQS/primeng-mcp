use std::sync::Arc;
use tokio::sync::{Mutex, Semaphore};

use super::model::{DocItem, DocItemKind, SyncOptions, SyncSummary};
use super::parser::{inject_frontmatter, parse_llms_txt};
use super::url::{normalize_version_dir, resolve_base_url, resolve_site_domain};

/// Executa a rotina completa de download e persistência offline da documentação.
pub async fn run_sync(options: SyncOptions) -> Result<SyncSummary, Box<dyn std::error::Error>> {
    let site_domain = resolve_site_domain(&options.version).map_err(|e| e)?;
    let base_url = resolve_base_url(&options.version).map_err(|e| e)?;
    let version_dir_name = normalize_version_dir(&options.version);
    let target_dir = options.output_dir.join(&version_dir_name);
    let components_dir = target_dir.join("components");
    let guides_dir = target_dir.join("guides");

    tokio::fs::create_dir_all(&components_dir).await?;
    tokio::fs::create_dir_all(&guides_dir).await?;

    let client = reqwest::Client::builder()
        .user_agent("primeng-mcp-sync/0.1.0")
        .build()?;

    // 1. Baixar llms.txt
    let llms_txt_url = format!("{}llms.txt", base_url);
    tracing::info!("Baixando manifesto principal: {}", llms_txt_url);
    let resp = client.get(&llms_txt_url).send().await?.error_for_status()?;
    let manifest_content = resp.text().await?;
    tokio::fs::write(target_dir.join("llms.txt"), &manifest_content).await?;

    // 2. Baixar llms-full.txt
    let llms_full_url = format!("{}llms-full.txt", base_url);
    tracing::info!("Baixando manifesto completo: {}", llms_full_url);
    let full_manifest_downloaded = match client.get(&llms_full_url).send().await {
        Ok(r) if r.status().is_success() => {
            if let Ok(content) = r.text().await {
                let _ = tokio::fs::write(target_dir.join("llms-full.txt"), content).await;
                true
            } else {
                false
            }
        }
        _ => false,
    };

    // 3. Parsear itens do llms.txt
    let items = parse_llms_txt(&manifest_content);
    let (guides, components): (Vec<DocItem>, Vec<DocItem>) = items
        .into_iter()
        .partition(|i| i.kind == DocItemKind::Guide);

    let total_guides = guides.len();
    let total_components = components.len();
    let semaphore = Arc::new(Semaphore::new(options.concurrency.max(1)));
    let failures = Arc::new(Mutex::new(Vec::<(String, String)>::new()));
    let comp_success = Arc::new(Mutex::new(0usize));
    let guide_success = Arc::new(Mutex::new(0usize));
    let version_tag = options.version.clone();

    tracing::info!(
        "Iniciando download concorrente de {} componentes e {} guias (concorrência máxima: {})...",
        total_components,
        total_guides,
        options.concurrency
    );

    let mut handles = Vec::new();

    // 4. Download de Componentes
    for comp in components {
        let client = client.clone();
        let sem = semaphore.clone();
        let fail = failures.clone();
        let succ = comp_success.clone();
        let comp_url = comp.component_markdown_url(&base_url);
        let out_path = components_dir.join(format!("{}.md", comp.slug()));
        let slug = comp.slug().to_string();
        let title = comp.title.clone();
        let desc = comp.description.clone();
        let ver = version_tag.clone();

        let handle = tokio::spawn(async move {
            let _permit = match sem.acquire().await {
                Ok(p) => p,
                Err(e) => {
                    fail.lock().await.push((slug, format!("Falha no semáforo: {}", e)));
                    return;
                }
            };

            match client.get(&comp_url).send().await {
                Ok(resp) => {
                    if resp.status().is_success() {
                        match resp.text().await {
                            Ok(content) => {
                                let formatted = inject_frontmatter(
                                    &content,
                                    &title,
                                    "component",
                                    &ver,
                                    &comp_url,
                                    &desc,
                                );
                                if let Err(e) = tokio::fs::write(&out_path, formatted).await {
                                    fail.lock().await.push((slug, format!("Erro de escrita em disco: {}", e)));
                                } else {
                                    *succ.lock().await += 1;
                                }
                            }
                            Err(e) => {
                                fail.lock().await.push((slug, format!("Erro ao ler corpo da resposta: {}", e)));
                            }
                        }
                    } else {
                        fail.lock().await.push((slug, format!("HTTP status {}", resp.status())));
                    }
                }
                Err(e) => {
                    fail.lock().await.push((slug, format!("Erro de rede: {}", e)));
                }
            }
        });

        handles.push(handle);
    }

    // 5. Download de Guias com suporte a múltiplos endpoints candidatos
    for guide in guides {
        let client = client.clone();
        let sem = semaphore.clone();
        let fail = failures.clone();
        let succ = guide_success.clone();
        let candidate_urls = guide.guide_markdown_urls(&base_url, &site_domain);
        let guide_rel_path = format!("{}.md", guide.path());
        let out_path = guides_dir.join(&guide_rel_path);
        let title = guide.title.clone();
        let desc = guide.description.clone();
        let ver = version_tag.clone();

        let handle = tokio::spawn(async move {
            let _permit = match sem.acquire().await {
                Ok(p) => p,
                Err(e) => {
                    fail.lock().await.push((title, format!("Falha no semáforo: {}", e)));
                    return;
                }
            };

            if let Some(parent) = out_path.parent() {
                let _ = tokio::fs::create_dir_all(parent).await;
            }

            let mut downloaded = false;
            let mut last_error = String::from("Nenhum endpoint disponível");

            for guide_url in &candidate_urls {
                match client.get(guide_url).send().await {
                    Ok(resp) if resp.status().is_success() => {
                        match resp.text().await {
                            Ok(content) => {
                                if content.trim_start().starts_with("<!doctype html")
                                    || content.trim_start().starts_with("<!DOCTYPE html")
                                {
                                    last_error = format!("HTML recebido de {}", guide_url);
                                } else {
                                    let formatted = inject_frontmatter(
                                        &content,
                                        &title,
                                        "guide",
                                        &ver,
                                        guide_url,
                                        &desc,
                                    );
                                    if let Err(e) = tokio::fs::write(&out_path, formatted).await {
                                        last_error = format!("Erro de escrita em disco: {}", e);
                                    } else {
                                        downloaded = true;
                                        break;
                                    }
                                }
                            }
                            Err(e) => {
                                last_error = format!("Erro ao ler corpo de {}: {}", guide_url, e);
                            }
                        }
                    }
                    Ok(resp) => {
                        last_error = format!("HTTP status {} em {}", resp.status(), guide_url);
                    }
                    Err(e) => {
                        last_error = format!("Erro de rede em {}: {}", guide_url, e);
                    }
                }
            }

            if downloaded {
                *succ.lock().await += 1;
            } else {
                fail.lock().await.push((title, last_error));
            }
        });

        handles.push(handle);
    }

    for handle in handles {
        let _ = handle.await;
    }

    let downloaded_components = *comp_success.lock().await;
    let downloaded_guides = *guide_success.lock().await;
    let final_failures = failures.lock().await.clone();

    Ok(SyncSummary {
        version: options.version,
        base_url,
        target_dir,
        manifest_downloaded: true,
        full_manifest_downloaded,
        total_components,
        downloaded_components,
        total_guides,
        downloaded_guides,
        failures: final_failures,
    })
}
