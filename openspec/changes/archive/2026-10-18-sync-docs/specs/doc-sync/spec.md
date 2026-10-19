# Spec Delta

## Purpose

Esta capacidade é responsável por sincronizar, baixar e organizar localmente a documentação versionada do PrimeNG em formato Markdown a partir dos endpoints oficiais do padrão LLMs.txt.

## ADDED Requirements

### Requirement: Versões suportadas limitadas ao PrimeNG 21 ou superior
O sistema SHALL aceitar apenas "latest" ou versões numéricas iguais ou superiores a 21, pois o padrão LLMs.txt do PrimeNG só existe a partir da v21. Versões anteriores não possuem os endpoints `/llms/` necessários.

#### Scenario: Rejeição de versão inferior a 21
- **WHEN** o usuário especificar `--version 20` ou qualquer versão numérica inferior a 21
- **THEN** o sistema exibe mensagem de erro informando que apenas versões 21+ são suportadas e encerra com código de saída diferente de zero, sem realizar nenhuma requisição HTTP.

#### Scenario: Aceitação de versão 21 ou superior
- **WHEN** o usuário especificar `--version 21` ou qualquer versão numérica >= 21
- **THEN** o sistema prossegue normalmente com a resolução de URL e sincronização.

### Requirement: Resolução de URL base conforme versão
O sistema SHALL resolver a URL base do PrimeNG de acordo com a versão solicitada: `https://primeng.dev/llms/` para "latest" e `https://v{N}.primeng.org/llms/` para versões numéricas (N >= 21).

#### Scenario: URL base para versão latest
- **WHEN** a versão solicitada for "latest"
- **THEN** o sistema utiliza `https://primeng.dev/llms/` como base para todas as requisições.

#### Scenario: URL base para versão numérica
- **WHEN** a versão solicitada for "21"
- **THEN** o sistema utiliza `https://v21.primeng.org/llms/` como base para todas as requisições.

### Requirement: Baixar e analisar o manifesto da documentação
O sistema SHALL baixar o manifesto `llms.txt` a partir da URL base resolvida e extrair a lista de componentes e guias disponíveis com seus respectivos links e títulos.

#### Scenario: Leitura bem-sucedida do manifesto
- **WHEN** o utilitário de sincronização for executado com uma versão válida
- **THEN** o sistema requisita `{url_base}/llms.txt`, identifica as seções "Guides" e "Components" e extrai todos os itens listados.

#### Scenario: Falha de conexão ou HTTP 404 no manifesto
- **WHEN** o utilitário tentar baixar o manifesto de uma versão inexistente ou sem conectividade
- **THEN** o sistema exibe mensagem de erro explicativa na saída padrão de erro e encerra com código de saída diferente de zero.

### Requirement: Download e armazenamento estruturado dos arquivos Markdown
O sistema SHALL baixar individualmente cada arquivo Markdown de componente e guia identificado no manifesto e salvá-lo na pasta local correspondente à versão.

#### Scenario: Download dos arquivos de componentes
- **WHEN** a lista de componentes for extraída do manifesto
- **THEN** o sistema baixa cada arquivo `.md` de componente (ex: `button.md`) e o persiste no caminho `docs/v{version}/components/{nome}.md`.

#### Scenario: Download dos arquivos de guias
- **WHEN** a lista de guias for extraída do manifesto
- **THEN** o sistema baixa cada guia disponível e o persiste no caminho `docs/v{version}/guides/{nome}.md`.

### Requirement: Suporte à seleção de versão via linha de comando
O sistema SHALL permitir a especificação da versão do PrimeNG a ser sincronizada através de argumento ou flag de linha de comando, assumindo "latest" por padrão.

#### Scenario: Execução sem especificar versão
- **WHEN** o comando for executado sem parâmetros de versão
- **THEN** o sistema assume automaticamente "latest", requisita de `https://primeng.dev/llms/` e salva os dados em `docs/latest/`.

#### Scenario: Execução especificando versão numérica
- **WHEN** o comando for executado com `--version 21`
- **THEN** o sistema requisita de `https://v21.primeng.org/llms/` e salva os dados em `docs/v21/`.

### Requirement: Resiliência a falhas de rede e relatório de sincronização
O sistema SHALL continuar o processo mesmo se itens individuais falharem, registrando os erros e exibindo ao final um resumo estatístico da sincronização.

#### Scenario: Resumo final após sincronização
- **WHEN** o processo de download de todos os itens do manifesto for concluído
- **THEN** o sistema exibe o total de componentes e guias processados com sucesso, a quantidade de arquivos salvos e lista quaisquer falhas ocorridas.
