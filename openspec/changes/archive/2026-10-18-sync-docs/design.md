# Design

## Context

O projeto `primeng-mcp` tem como objetivo expor a documentação do PrimeNG para assistentes de IA de forma 100% offline em runtime. Para viabilizar isso, a documentação oficial é disponibilizada em dois padrões de URL: `https://primeng.dev/llms/` para a versão mais recente (latest) e `https://v{N}.primeng.org/llms/` para versões específicas. Esses endpoints precisam ser baixados previamente e versionados no projeto sob o diretório `docs/{version}/`. Veja `proposal.md` para a motivação.

## Goals / Non-Goals

**Goals:**
- Implementar um utilitário CLI em Rust (`src/bin/sync_docs.rs`) para download e sincronização da documentação.
- Baixar o manifesto `llms.txt`, interpretar os links de componentes e guias, e baixar cada arquivo Markdown correspondente.
- Armazenar os arquivos de forma organizada em `docs/v{version}/components/` e `docs/v{version}/guides/`.
- Suportar download concorrente com controle de vazão (throttling/semaphore) para evitar sobrecarga ou bloqueio (HTTP 429).
- Fornecer feedback visual em tempo de execução via `tracing`/console.

**Non-Goals:**
- Não inclui a implementação do servidor MCP ou das suas ferramentas neste momento (será abordado na spec subsequente).
- Não realiza indexação ou vetorização dos arquivos baixados neste passo (os arquivos brutos em Markdown serão a fonte da verdade).

## Decisions

### Decisão 1: Binário dedicado (`src/bin/sync_docs.rs`)
- **Escolha**: Criar um arquivo executável separado sob `src/bin/sync_docs.rs` executável via `cargo run --bin sync-docs`.
- **Justificativa**: Mantém o binário principal `primeng-mcp` enxuto e otimizado apenas para a execução do servidor MCP via `stdio` em tempo de execução, separando claramente as responsabilidades de desenvolvimento/manutenção das de runtime.
- **Alternativas consideradas**: Embutir como um subcomando `primeng-mcp sync`. Foi descartado inicialmente para evitar inclusão desnecessária de dependências e código de scraping no runtime final do servidor MCP caso se opte por compilações separadas.

### Decisão 2: Cliente HTTP assíncrono com Tokio e Reqwest
- **Escolha**: Usar `reqwest` com a feature `rustls-tls` (evitando dependências de OpenSSL do sistema operacional C) integrada ao runtime `tokio`.
- **Justificativa**: Como o projeto já adota `tokio`, o `reqwest` se integra perfeitamente para downloads assíncronos não bloqueantes.
- **Alternativas consideradas**: `ureq` (síncrono) ou `curl`. `reqwest` oferece tipagem estrita, robustez e alta performance para downloads paralelos.

### Decisão 3: Concorrência com semáforo (`tokio::sync::Semaphore`)
- **Escolha**: Limitar a quantidade de downloads simultâneos (ex: 5 a 8 conexões simultâneas) utilizando um semáforo do Tokio.
- **Justificativa**: Evita disparo excessivo de requisições que poderiam causar conexões recusadas ou HTTP 429 (Too Many Requests) pelos servidores da infraestrutura do PrimeNG.
- **Alternativas consideradas**: Download puramente sequencial (muito lento para ~80 componentes) ou sem limite (risco de throttling).

## Risks / Trade-offs

- **[Mudança na estrutura de links do `llms.txt` do PrimeNG]** → O parser deve utilizar expressões regulares e buscas de prefixo resilientes para extrair a URL de destino de cada item em Markdown `[Título](URL)`.
- **[Queda de rede em arquivos individuais]** → Implementar retentativas (retry) simples e registrar ao final quais arquivos não puderam ser baixados, sem abortar a sincronização dos demais.
- **[Tamanho do repositório com múltiplas versões]** → Os arquivos em Markdown do PrimeNG v21 ocupam alguns megabytes no total, sendo leves e adequados para inclusão no repositório e embutimento via `rust-embed`.
