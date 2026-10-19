/// Normaliza a pasta de saída para a versão (ex: "latest" -> "latest", "21" -> "v21", "v21" -> "v21").
pub fn normalize_version_dir(version: &str) -> String {
    let lower = version.trim().to_lowercase();
    if lower == "latest" {
        "latest".to_string()
    } else if lower.starts_with('v') {
        lower
    } else {
        format!("v{}", lower)
    }
}

/// Resolve o domínio base do site para a versão especificada (sem sufixo /llms/).
/// "latest" -> "https://primeng.dev"
/// "21" / "v21" -> "https://v21.primeng.org"
pub fn resolve_site_domain(version: &str) -> Result<String, String> {
    let normalized = version.trim().to_lowercase();
    if normalized == "latest" {
        return Ok("https://primeng.dev".to_string());
    }

    let num_str = normalized.strip_prefix('v').unwrap_or(&normalized);
    match num_str.parse::<u32>() {
        Ok(num) => {
            if num < 21 {
                Err(format!(
                    "Apenas PrimeNG versão 21 ou superior é suportado (fornecido: '{}'). Versões anteriores não possuem suporte ao padrão LLMs.txt.",
                    version
                ))
            } else {
                Ok(format!("https://v{}.primeng.org", num))
            }
        }
        Err(_) => Err(format!(
            "Versão inválida: '{}'. Use 'latest' ou um número de versão maior ou igual a 21 (ex: '21', 'v21').",
            version
        )),
    }
}

/// Resolve a URL base para a documentação do PrimeNG baseada na versão.
/// Suporta "latest" (https://primeng.dev/llms/) e versões numéricas >= 21 (https://v{N}.primeng.org/llms/).
pub fn resolve_base_url(version: &str) -> Result<String, String> {
    let site_domain = resolve_site_domain(version)?;
    Ok(format!("{}/llms/", site_domain))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normalize_version_dir() {
        assert_eq!(normalize_version_dir("latest"), "latest");
        assert_eq!(normalize_version_dir("LATEST"), "latest");
        assert_eq!(normalize_version_dir("21"), "v21");
        assert_eq!(normalize_version_dir("v21"), "v21");
        assert_eq!(normalize_version_dir("V21"), "v21");
    }

    #[test]
    fn test_resolve_site_domain() {
        assert_eq!(resolve_site_domain("latest").unwrap(), "https://primeng.dev");
        assert_eq!(resolve_site_domain("21").unwrap(), "https://v21.primeng.org");
        assert_eq!(resolve_site_domain("v21").unwrap(), "https://v21.primeng.org");
        assert_eq!(resolve_site_domain("22").unwrap(), "https://v22.primeng.org");
        assert!(resolve_site_domain("20").is_err());
    }

    #[test]
    fn test_resolve_base_url_latest() {
        assert_eq!(
            resolve_base_url("latest").unwrap(),
            "https://primeng.dev/llms/"
        );
        assert_eq!(
            resolve_base_url("LATEST").unwrap(),
            "https://primeng.dev/llms/"
        );
    }

    #[test]
    fn test_resolve_base_url_numeric() {
        assert_eq!(
            resolve_base_url("21").unwrap(),
            "https://v21.primeng.org/llms/"
        );
        assert_eq!(
            resolve_base_url("v21").unwrap(),
            "https://v21.primeng.org/llms/"
        );
        assert_eq!(
            resolve_base_url("22").unwrap(),
            "https://v22.primeng.org/llms/"
        );
    }

    #[test]
    fn test_resolve_base_url_rejects_below_21() {
        assert!(resolve_base_url("20").is_err());
        assert!(resolve_base_url("v20").is_err());
        assert!(resolve_base_url("19").is_err());
        assert!(resolve_base_url("18").is_err());
    }

    #[test]
    fn test_resolve_base_url_invalid_string() {
        assert!(resolve_base_url("abc").is_err());
        assert!(resolve_base_url("").is_err());
    }
}
