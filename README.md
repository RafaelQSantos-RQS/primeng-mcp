# PrimeNG MCP Server

Um servidor **MCP (Model Context Protocol)** em Rust para consulta rápida, eficiente e **100% offline** da documentação do [PrimeNG](https://primeng.org/) (biblioteca de componentes UI para Angular).

Projetado para integração com assistentes de IA e IDEs como Cursor, Claude Desktop, Antigravity, Windsurf e qualquer cliente compatível com MCP.

---

## 📌 Requisitos e Versões Suportadas

O projeto consome as especificações do padrão `llms.txt` oficial do PrimeNG, disponibilizado a partir da **versão 21**.

* **PrimeNG v21+** (`https://v{N}.primeng.org/llms/`)
* **PrimeNG Latest** (`https://primeng.dev/llms/`)

*(Versões anteriores à 21 não possuem suporte a este padrão e são rejeitadas pelo sistema).*

---

## 🚀 Sincronização de Documentação Offline (`sync-docs`)

O projeto inclui um utilitário CLI para baixar e atualizar os arquivos da documentação oficial em formato Markdown puro, já enriquecidos com metadados em **YAML frontmatter** (título, tipo, versão, URL de origem e descrição resumida):

### Sincronizar versão específica (ex: v21)
```bash
cargo run --bin sync-docs -- --version 21
```

### Sincronizar versão mais recente (`latest`)
```bash
cargo run --bin sync-docs -- --version latest
```

### Opções disponíveis
```bash
cargo run --bin sync-docs -- --help
```
* `-V, --version <VERSION>`: Versão do PrimeNG (`latest`, `21`, `v21`). [Padrão: `latest`]
* `-o, --output <OUTPUT>`: Diretório local onde os arquivos serão salvos. [Padrão: `docs`]
* `-c, --concurrency <N>`: Número máximo de downloads simultâneos. [Padrão: `8`]

---

## 📁 Estrutura da Base de Documentação

Os arquivos sincronizados ficam versionados localmente sob a pasta `docs/`:

```text
docs/
├── v21/
│   ├── llms.txt               # Manifesto principal de índices
│   ├── llms-full.txt          # Compilação integral da documentação
│   ├── components/            # 96+ componentes em Markdown com frontmatter
│   │   ├── button.md
│   │   ├── table.md
│   │   └── ...
│   └── guides/                # Guias oficiais com frontmatter
│       ├── installation.md
│       ├── configuration.md
│       ├── theming/styled.md
│       └── migration/v21.md
└── latest/
    ├── llms.txt
    ├── llms-full.txt
    ├── components/
    └── guides/
```

---

## 🧪 Testes

Para executar a suite de testes unitários:

```bash
cargo test
```

---

## 🗺️ Roadmap

- [x] Extrator e sincronizador de documentação offline (`sync-docs`)
- [x] Resolução automática de URLs e enriquecimento com YAML frontmatter
- [ ] Servidor MCP via `stdio` usando o SDK oficial `rmcp`
- [ ] Ferramentas MCP (`list_components`, `get_component`, `get_guide`, `search_docs`)
- [ ] Embutimento de dados no binário final via `rust-embed` para distribuição single-binary
