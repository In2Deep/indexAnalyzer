//! Unit test for provider selection fallback logic in vectorize command.

struct Vectorize {
    provider: Option<String>,
}

struct AppConfig {
    default_provider: String,
}

/// This is the function under test. It does not exist yet and will fail to compile.
// For error handling, assume select_provider_id will return Result<String, ProviderError> in the future.
#[derive(Debug, PartialEq)]
pub struct ProviderError;

fn select_provider_id(vectorize: &Vectorize, config: &AppConfig) -> Result<String, ProviderError> {
    match &vectorize.provider {
        Some(p) if !p.is_empty() => Ok(p.clone()),
        _ if !config.default_provider.is_empty() => Ok(config.default_provider.clone()),
        _ => Err(ProviderError),
    }
}

#[cfg(test)]
mod error_tests {
    use super::*;

    #[derive(Debug, PartialEq)]
    pub struct ProviderError;

    // This test will fail until select_provider_id returns Result
    #[test]
    fn test_provider_error_if_none_and_config_empty() {
        let vectorize = Vectorize {
            provider: None,
        };
        let config = AppConfig {
            default_provider: "".to_string(),
        };
        let result = select_provider_id(&vectorize, &config);
        assert!(result.is_err(), "select_provider_id should return an error if no provider specified");
    }
}


#[test]
fn test_provider_fallback_to_config_default() {
    let vectorize = Vectorize { provider: None };
    let config = AppConfig { default_provider: "default_provider_id_from_config".to_string() };
    let selected = select_provider_id(&vectorize, &config);
    assert_eq!(selected, Ok("default_provider_id_from_config".to_string()));
}

#[test]
fn test_provider_cli_overrides_config() {
    let vectorize = Vectorize {
        provider: Some("cli_provider".to_string()),
    };
    let config = AppConfig {
        default_provider: "config_default".to_string(),
    };
    let selected = select_provider_id(&vectorize, &config);
    assert_eq!(selected, Ok("cli_provider".to_string()));
}
