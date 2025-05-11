//! RED test for config-driven provider/model selection and error handling

use indexer::config::AppConfig;

#[test]
fn test_provider_selection_from_config() {
    let mut config = AppConfig::default();
    config.global_defaults = Some(indexer::config::GlobalDefaults {
        provider: "openai".to_string(),
        db: "redis".to_string(),
    });
    let provider = config.global_defaults.as_ref().unwrap().provider.clone();
    assert_eq!(provider, "openai");
}

#[test]
fn test_provider_missing_in_config() {
    let config = AppConfig::default();
    let result = config.global_defaults.as_ref().map(|gd| gd.provider.clone());
    assert!(result.is_none(), "Should be None if provider missing in config");
}
