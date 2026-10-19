use super::model::{DocItem, DocItemKind};

/// Extrai o título e o primeiro parágrafo de descrição contidos entre o primeiro `# ` e o primeiro `## `.
pub fn extract_markdown_meta(content: &str) -> (Option<String>, Option<String>) {
    let mut title = None;
    let mut description_lines = Vec::new();
    let mut inside_header = false;

    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("# ") && title.is_none() {
            title = Some(trimmed[2..].trim().to_string());
            inside_header = true;
            continue;
        }

        if inside_header {
            if trimmed.starts_with("## ") {
                break;
            }
            if !trimmed.is_empty() {
                description_lines.push(trimmed);
            }
        }
    }

    let description = if description_lines.is_empty() {
        None
    } else {
        Some(description_lines.join(" "))
    };

    (title, description)
}

/// Injeta um frontmatter YAML no topo do documento Markdown.
pub fn inject_frontmatter(
    content: &str,
    fallback_title: &str,
    doc_type: &str,
    version: &str,
    source_url: &str,
    fallback_description: &str,
) -> String {
    let body = if content.starts_with("---\n") {
        if let Some(end_pos) = content[4..].find("\n---\n") {
            content[4 + end_pos + 5..].trim_start()
        } else {
            content
        }
    } else {
        content
    };

    let (extracted_title, extracted_desc) = extract_markdown_meta(body);
    let final_title = extracted_title.as_deref().unwrap_or(fallback_title);
    let final_desc = extracted_desc
        .as_deref()
        .filter(|s| !s.is_empty())
        .unwrap_or(fallback_description);

    let safe_title = final_title.replace('"', "\\\"");
    let safe_desc = final_desc.replace('"', "\\\"");
    let safe_url = source_url.replace('"', "\\\"");

    format!(
        "---\ntitle: \"{}\"\ntype: \"{}\"\nversion: \"{}\"\nsource_url: \"{}\"\ndescription: \"{}\"\n---\n\n{}",
        safe_title, doc_type, version, safe_url, safe_desc, body.trim_start()
    )
}

/// Analisa o conteúdo textual do arquivo `llms.txt` e extrai os itens das seções "Guides" e "Components".
pub fn parse_llms_txt(content: &str) -> Vec<DocItem> {
    let mut items = Vec::new();
    let mut current_kind: Option<DocItemKind> = None;

    for line in content.lines() {
        let trimmed = line.trim();

        if trimmed.starts_with("## Guides") {
            current_kind = Some(DocItemKind::Guide);
            continue;
        } else if trimmed.starts_with("## Components") {
            current_kind = Some(DocItemKind::Component);
            continue;
        } else if trimmed.starts_with("## ") {
            current_kind = None;
            continue;
        }

        let kind = match &current_kind {
            Some(k) => k.clone(),
            None => continue,
        };

        if !trimmed.starts_with("- [") {
            continue;
        }

        if let Some(open_bracket) = trimmed.find('[') {
            if let Some(close_bracket) = trimmed[open_bracket..].find(']') {
                let close_bracket = open_bracket + close_bracket;
                let title = &trimmed[open_bracket + 1..close_bracket];

                let rest = &trimmed[close_bracket + 1..];
                if let Some(open_paren) = rest.find('(') {
                    if let Some(close_paren) = rest[open_paren..].find(')') {
                        let close_paren = open_paren + close_paren;
                        let url = &rest[open_paren + 1..close_paren];

                        let after_url = rest[close_paren + 1..].trim();
                        let description = after_url.strip_prefix(':').unwrap_or(after_url).trim();

                        items.push(DocItem {
                            title: title.to_string(),
                            url: url.to_string(),
                            description: description.to_string(),
                            kind,
                        });
                    }
                }
            }
        }
    }

    items
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_markdown_meta() {
        let sample = r#"
# Angular Button Component

Button is an extension to standard button element with icons and theming.

## Accessibility
Screen reader notes...
"#;
        let (title, desc) = extract_markdown_meta(sample);
        assert_eq!(title.as_deref(), Some("Angular Button Component"));
        assert_eq!(
            desc.as_deref(),
            Some("Button is an extension to standard button element with icons and theming.")
        );
    }

    #[test]
    fn test_inject_frontmatter() {
        let sample = r#"# Installation

Setting up PrimeNG in an Angular CLI project.

## Download-
npm install primeng
"#;
        let injected = inject_frontmatter(
            sample,
            "Installation",
            "guide",
            "21",
            "https://v21.primeng.org/installation.md",
            "Fallback desc",
        );

        assert!(injected.starts_with("---\n"));
        assert!(injected.contains("title: \"Installation\""));
        assert!(injected.contains("type: \"guide\""));
        assert!(injected.contains("version: \"21\""));
        assert!(injected.contains("source_url: \"https://v21.primeng.org/installation.md\""));
        assert!(injected.contains("description: \"Setting up PrimeNG in an Angular CLI project.\""));
        assert!(injected.contains("# Installation"));

        let reinjected = inject_frontmatter(
            &injected,
            "Installation",
            "guide",
            "latest",
            "https://primeng.dev/installation.md",
            "Fallback desc",
        );
        assert_eq!(reinjected.matches("---").count(), 2);
        assert!(reinjected.contains("version: \"latest\""));
    }

    #[test]
    fn test_parse_llms_txt_extracts_guides_and_components() {
        let sample = r#"
# PrimeNG

> The Most Complete Angular UI Component Library

## Guides

- [Installation](https://primeng.org/installation): Setting up PrimeNG in an Angular CLI project.
- [Configuration](https://primeng.org/configuration): Application wide configuration for PrimeNG.

## Components

- [Angular Accordion Component](https://primeng.org/accordion): Accordion groups a collection of contents in tabs.
- [Angular Button Component](https://primeng.org/button): Button is an extension to standard button element with icons and theming.
"#;

        let items = parse_llms_txt(sample);
        assert_eq!(items.len(), 4);

        assert_eq!(items[0].kind, DocItemKind::Guide);
        assert_eq!(items[0].title, "Installation");
        assert_eq!(items[0].url, "https://primeng.org/installation");
        assert_eq!(items[0].slug(), "installation");
        assert_eq!(items[0].path(), "installation");
        assert_eq!(
            items[0].description,
            "Setting up PrimeNG in an Angular CLI project."
        );

        assert_eq!(items[1].kind, DocItemKind::Guide);
        assert_eq!(items[1].slug(), "configuration");

        assert_eq!(items[2].kind, DocItemKind::Component);
        assert_eq!(items[2].title, "Angular Accordion Component");
        assert_eq!(items[2].slug(), "accordion");

        assert_eq!(items[3].kind, DocItemKind::Component);
        assert_eq!(items[3].title, "Angular Button Component");
        assert_eq!(items[3].slug(), "button");
        assert_eq!(
            items[3].component_markdown_url("https://v21.primeng.org/llms/"),
            "https://v21.primeng.org/llms/components/button.md"
        );
    }

    #[test]
    fn test_parse_llms_txt_handles_empty_lines_and_unknown_sections() {
        let sample = r#"
# Heading

Random text without structure

## Unknown Section
- [Ignored](https://example.com/ignored): should be ignored

## Components
- [Valid](https://primeng.org/valid): valid component

"#;
        let items = parse_llms_txt(sample);
        assert_eq!(items.len(), 1);
        assert_eq!(items[0].slug(), "valid");
        assert_eq!(items[0].kind, DocItemKind::Component);
    }
}
