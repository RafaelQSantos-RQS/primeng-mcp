# Proposal

## Why

Para que o servidor MCP do PrimeNG funcione de forma primariamente offline, rápida e autocontida, é necessário possuir uma cópia local estruturada da documentação de componentes e guias. A documentação oficial do PrimeNG agora expõe manifestos `llms.txt` e páginas em Markdown puro (`.md`) para consumo por LLMs. Uma ferramenta de sincronização (`sync-docs`) automatiza a coleta, validação e armazenamento dessa documentação por versão no repositório, viabilizando o embutimento dos dados no binário final e suporte a múltiplas versões (iniciando pela v21).

## What Changes

- Criação de um utilitário CLI dedicado (`sync_docs` ou `cargo run --bin sync-docs`).
- Suporte a download automatizado a partir dos endpoints LLMs do PrimeNG (`https://v{version}.primeng.org/llms/llms.txt`).
- Parsing do arquivo `llms.txt` para extrair links e metadados de componentes e guias.
- Download concorrente e resiliente dos arquivos `.md` individuais de cada componente e guia.
- Estruturação dos arquivos no sistema de arquivos local sob `docs/v{version}/components/` e `docs/v{version}/guides/`.
- Suporte a seleção de versão via argumento CLI (padrão: `21`).
- Exibição de sumário e métricas da sincronização (total de arquivos baixados, bytes e falhas, se houver).

## Capabilities

### New Capabilities
- `doc-sync`: Sincronização, download e persistência local da documentação versionada do PrimeNG em formato Markdown a partir dos endpoints oficiais otimizados para LLMs.

### Modified Capabilities
*(Nenhuma - primeira especificação do projeto)*

## Impact

- **Código e Binários**: Adição de um novo binário executável em `src/bin/sync_docs.rs` ou subcomando CLI.
- **Armazenamento**: Criação do diretório `docs/` e subdiretórios versionados (ex: `docs/v21/components/`, `docs/v21/guides/`).
- **Dependências**: Requer bibliotecas para requisições HTTP (`reqwest` com TLS rustls ou `tokio` client) no `Cargo.toml`.
