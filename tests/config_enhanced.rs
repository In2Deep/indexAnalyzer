//! Tests for enhanced config system loading (TDD: Phase 2 - RED)

use indexer::config::{AppConfig, ConfigError};
use std::fs;


#[test]
fn test_load_config_with_global_defaults() {
    let temp_dir = tempfile::tempdir().unwrap();
    let home = temp_dir.path();
    std::env::set_var("HOME", home);
    let config_dir = home.join(".indexer");
    let config_path = config_dir.join("config.yaml");
    let _ = fs::remove_file(&config_path);
    let config = AppConfig::load().unwrap_or_else(|_| AppConfig::default());
    assert_eq!(config.redis_url, Some("redis://127.0.0.1:6379/0".to_string()));
    assert_eq!(config.log_level, Some("info".to_string()));
    let _ = fs::remove_file(&config_path);
    std::env::remove_var("HOME");
    temp_dir.close().unwrap();
}

#[test]
fn test_load_config_from_file() {
    let temp_dir = tempfile::tempdir().unwrap();
    let home = temp_dir.path();
    std::env::set_var("HOME", home);
    let config_dir = home.join(".indexer");
    let config_path = config_dir.join("config.yaml");
    fs::create_dir_all(&config_dir).unwrap();
    println!("DEBUG: HOME={:?}", std::env::var("HOME"));
    println!("DEBUG: home exists? {}", home.exists());
    println!("DEBUG: home contents: {:?}", fs::read_dir(home).map(|rd| rd.map(|e| e.map(|e| e.path())).collect::<Result<Vec<_>,_>>()).unwrap_or_default());
    #[derive(serde::Serialize)]
    struct TestConfig<'a> {
        redis_url: &'a str,
        log_level: &'a str,
    }
    let yaml_struct = serde_yaml::to_string(&TestConfig {
        redis_url: "redis://custom:6379/1",
        log_level: "debug",
    }).unwrap();
    fs::write(&config_path, yaml_struct).unwrap();
    println!("DEBUG: config_path exists? {}", config_path.exists());
    let config = AppConfig::load().unwrap();
    assert_eq!(config.redis_url, Some("redis://custom:6379/1".to_string()));
    assert_eq!(config.log_level, Some("debug".to_string()));
    std::env::remove_var("HOME");
    let _ = fs::remove_file(&config_path);
    temp_dir.close().unwrap();
}

#[test]
fn test_load_config_missing_home() {
    use std::fs;
    std::env::remove_var("HOME");
    std::env::remove_var("XDG_CONFIG_HOME");
    // Attempt to remove any lingering config.yaml
    if let Some(home) = dirs_next::home_dir() {
        let config_path = home.join(".indexer").join("config.yaml");
        let _ = fs::remove_file(&config_path);
    }
    let result = AppConfig::load();
    assert!(matches!(result, Err(ConfigError::HomeDirNotFound)), "Expected HomeDirNotFound error, got: {:?}", result);
}

#[test]
fn test_load_config_bad_yaml() {
    let temp_dir = tempfile::tempdir().unwrap();
    let home = temp_dir.path();
    std::env::set_var("HOME", home);
    let config_dir = home.join(".indexer");
    let config_path = config_dir.join("config.yaml");
    fs::create_dir_all(&config_dir).unwrap();
    fs::write(&config_path, "this is not yaml: [").unwrap();
    assert!(config_path.exists(), "config.yaml was not written");
    let result = AppConfig::load();
    match result {
        Err(ConfigError::Yaml(_)) => {},
        other => panic!("Expected ConfigError::Yaml(_), got: {:?}", other),
    }
    std::env::remove_var("HOME");
    let _ = fs::remove_file(&config_path);
    temp_dir.close().unwrap();
}
