use std::path::PathBuf;

/// Tipo de item listado no manifesto da documentação.
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum DocItemKind {
    Guide,
    Component,
}

/// Representa um item da documentação extraído do llms.txt.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct DocItem {
    pub title: String,
    pub url: String,
    pub description: String,
    pub kind: DocItemKind,
}

impl DocItem {
    /// Extrai o caminho relativo da URL do item.
    /// Ex: "https://primeng.org/installation" -> "installation"
    /// Ex: "https://primeng.org/theming/styled" -> "theming/styled"
    pub fn path(&self) -> &str {
        if let Some(pos) = self.url.find("://") {
            let without_proto = &self.url[pos + 3..];
            if let Some(slash_pos) = without_proto.find('/') {
                return without_proto[slash_pos + 1..].trim_matches('/');
            }
        }
        self.url.trim_matches('/')
    }

    /// Extrai o slug a partir da URL do item (ex: https://primeng.org/button -> "button").
    pub fn slug(&self) -> &str {
        self.url
            .trim_end_matches('/')
            .rsplit('/')
            .next()
            .unwrap_or(&self.title)
    }

    /// Retorna a URL do arquivo Markdown no endpoint LLM para um componente.
    pub fn component_markdown_url(&self, base_url: &str) -> String {
        let base = base_url.trim_end_matches('/');
        format!("{}/components/{}.md", base, self.slug())
    }

    /// Retorna a URL do arquivo Markdown para um guia, ajustando o domínio da versão e adicionando a extensão .md.
    pub fn guide_markdown_url(&self, site_domain: &str) -> String {
        let domain = site_domain.trim_end_matches('/');
        let path = self.path();
        format!("{}/{}.md", domain, path)
    }

    /// Retorna as URLs candidatas para o download de um guia:
    /// 1. Padrão canônico de endpoints LLM: {base_url}pages/{slug}.md (ex: /llms/pages/v21.md, /llms/pages/installation.md)
    /// 2. Padrão direto no domínio com .md: {site_domain}/{path}.md (ex: /theming/styled.md, /installation.md)
    pub fn guide_markdown_urls(&self, base_url: &str, site_domain: &str) -> Vec<String> {
        let base = base_url.trim_end_matches('/');
        let domain = site_domain.trim_end_matches('/');
        let slug = self.slug();
        let path = self.path();

        let mut urls = Vec::new();
        urls.push(format!("{}/pages/{}.md", base, slug));
        let fallback = format!("{}/{}.md", domain, path);
        if !urls.contains(&fallback) {
            urls.push(fallback);
        }
        urls
    }
}

/// Opções para o processo de sincronização de documentação.
#[derive(Debug, Clone)]
pub struct SyncOptions {
    pub version: String,
    pub output_dir: PathBuf,
    pub concurrency: usize,
}

/// Resumo dos resultados da sincronização.
#[derive(Debug, Clone)]
pub struct SyncSummary {
    pub version: String,
    pub base_url: String,
    pub target_dir: PathBuf,
    pub manifest_downloaded: bool,
    pub full_manifest_downloaded: bool,
    pub total_components: usize,
    pub downloaded_components: usize,
    pub total_guides: usize,
    pub downloaded_guides: usize,
    pub failures: Vec<(String, String)>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_doc_item_path_and_guide_url() {
        let item1 = DocItem {
            title: "Installation".to_string(),
            url: "https://primeng.org/installation".to_string(),
            description: "".to_string(),
            kind: DocItemKind::Guide,
        };
        assert_eq!(item1.path(), "installation");
        assert_eq!(
            item1.guide_markdown_url("https://v21.primeng.org"),
            "https://v21.primeng.org/installation.md"
        );

        let item2 = DocItem {
            title: "Styled Mode".to_string(),
            url: "https://primeng.org/theming/styled".to_string(),
            description: "".to_string(),
            kind: DocItemKind::Guide,
        };
        assert_eq!(item2.path(), "theming/styled");
        assert_eq!(
            item2.guide_markdown_url("https://v21.primeng.org"),
            "https://v21.primeng.org/theming/styled.md"
        );
        assert_eq!(
            item2.guide_markdown_url("https://primeng.dev"),
            "https://primeng.dev/theming/styled.md"
        );

        let candidates = item2.guide_markdown_urls("https://v21.primeng.org/llms/", "https://v21.primeng.org");
        assert_eq!(candidates.len(), 2);
        assert_eq!(candidates[0], "https://v21.primeng.org/llms/pages/styled.md");
        assert_eq!(candidates[1], "https://v21.primeng.org/theming/styled.md");
    }
}
