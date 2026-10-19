use clap::Parser;
use primeng_mcp::sync::{run_sync, SyncOptions};
use std::path::PathBuf;
use std::process;

#[derive(Parser, Debug)]
#[command(
    name = "sync-docs",
    about = "Sincroniza a documentação offline do PrimeNG (versões 21+ ou latest)",
    disable_version_flag = true
)]
struct Cli {
    #[arg(
        short = 'V',
        long = "version",
        default_value = "latest",
        help = "Versão do PrimeNG a ser sincronizada (ex: latest, 21, v21). Suporta apenas v21+."
    )]
    version: String,

    #[arg(
        short,
        long,
        default_value = "docs",
        help = "Diretório de saída local para armazenar a documentação"
    )]
    output: PathBuf,

    #[arg(
        short,
        long,
        default_value = "8",
        help = "Número máximo de downloads simultâneos (controle de concorrência)"
    )]
    concurrency: usize,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
        )
        .init();

    let cli = Cli::parse();

    println!("============================================================");
    println!("  PrimeNG Documentation Sync (Offline Extractor)");
    println!("============================================================");
    println!("Versão selecionada: {}", cli.version);
    println!("Pasta de saída:     {}", cli.output.display());
    println!("Concorrência:       {} conexões simultâneas", cli.concurrency);
    println!("------------------------------------------------------------");

    let options = SyncOptions {
        version: cli.version.clone(),
        output_dir: cli.output,
        concurrency: cli.concurrency,
    };

    match run_sync(options).await {
        Ok(summary) => {
            println!("\n============================================================");
            println!("  Resumo da Sincronização");
            println!("============================================================");
            println!("Versão:                {}", summary.version);
            println!("URL Base:              {}", summary.base_url);
            println!("Diretório Local:       {}", summary.target_dir.display());
            println!(
                "Manifesto llms.txt:    {}",
                if summary.manifest_downloaded {
                    "✓ OK"
                } else {
                    "✗ Falhou"
                }
            );
            println!(
                "Manual llms-full.txt:  {}",
                if summary.full_manifest_downloaded {
                    "✓ OK"
                } else {
                    "✗ Não encontrado ou falhou"
                }
            );
            println!(
                "Componentes baixados:  {}/{}",
                summary.downloaded_components, summary.total_components
            );
            println!(
                "Guias baixados:        {}/{}",
                summary.downloaded_guides, summary.total_guides
            );

            if summary.failures.is_empty() {
                println!("\nStatus: Sincronização concluída com 100% de sucesso!");
            } else {
                println!(
                    "\nStatus: Sincronização concluída com {} falha(s):",
                    summary.failures.len()
                );
                for (item, err) in &summary.failures {
                    println!("  - {}: {}", item, err);
                }
            }
            println!("============================================================");
        }
        Err(e) => {
            eprintln!("\nErro na sincronização: {}", e);
            process::exit(1);
        }
    }
}
