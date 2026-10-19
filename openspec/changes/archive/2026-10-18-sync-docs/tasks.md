# Tasks

## 1. Configuração e Dependências

- [x] 1.1 Adicionar dependência `reqwest` com as features `rustls-tls` via cargo e verificar que `cargo check` compila com sucesso.
- [x] 1.2 Configurar estrutura para múltiplos binários criando `src/bin/sync_docs.rs` e validar compilação com `cargo check --bin sync-docs`.

## 2. Parser do Manifesto llms.txt

- [x] 2.1 Implementar função de resolução de URL base: `https://primeng.dev/llms/` para "latest" e `https://v{N}.primeng.org/llms/` para versões numéricas, e testar ambos os padrões com testes unitários.
- [x] 2.2 Implementar estruturas de dados e função de extração de itens e links das seções "Guides" e "Components" a partir do texto de `llms.txt`, com testes unitários cobrindo extração de links válidos e linhas vazias, verificando com `cargo test`.

## 3. Download Concorrente e Persistência

- [x] 3.1 Implementar rotina assíncrona de download de arquivos Markdown com controle de concorrência via `tokio::sync::Semaphore` e persistência no disco em `docs/{version}/` (`docs/latest/` ou `docs/v{N}/`).
- [x] 3.2 Integrar interface de linha de comando com `clap` permitindo passar `--version` (com padrão `latest`), exibindo logs de progresso e resumo final da sincronização.

## 4. Validação e Execução

- [x] 4.1 Executar a sincronização completa da documentação da versão 21 com `cargo run --bin sync-docs -- --version 21` e verificar a criação dos arquivos em `docs/v21/components/` e `docs/v21/guides/`.
- [x] 4.2 Executar `cargo test` para assegurar que todos os testes unitários e de integração passam.
