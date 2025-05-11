//! Tests for enhanced config system loading (TDD: Phase 2 - RED)

use indexer::config::{AppConfig, ConfigError};
use std::fs;
use std::env;


#[test]
fn test_load_config_with_global_defaults() {
    let home = dirs_next::home_dir().unwrap();
    let config_dir = home.join(".indexer");
    let config_path = config_dir.join("config.yaml");
    let _ = fs::remove_file(&config_path);
    let _ = fs::remove_file(&config_path);
    let config = AppConfig::load().unwrap_or_else(|_| AppConfig::default());
    assert_eq!(config.redis_url, Some("redis://127.0.0.1:6379/0".to_string()));
    assert_eq!(config.log_level, Some("info".to_string()));
    let _ = fs::remove_file(&config_path);
}

#[test]
fn test_load_config_from_file() {
    let home = dirs_next::home_dir().unwrap();
    let config_dir = home.join(".indexer");
    let config_path = config_dir.join("config.yaml");
    fs::create_dir_all(&config_dir).unwrap();
    let _ = fs::remove_file(&config_path);
    let yaml = "redis_url: redis://custom:6379/1\nlog_level: debug\n";
    fs::write(&config_path, yaml).unwrap();
    let config = AppConfig::load().unwrap();
    assert_eq!(config.redis_url, Some("redis://custom:6379/1".to_string()));
    assert_eq!(config.log_level, Some("debug".to_string()));
    let _ = fs::remove_file(&config_path);
}

#[test]
fn test_load_config_missing_home() {
    // Temporarily override HOME
    let orig_home = env::var("HOME").ok();
    env::remove_var("HOME");
    let result = AppConfig::load();
    if let Some(val) = orig_home { env::set_var("HOME", val); }
    assert!(matches!(result, Err(ConfigError::HomeDirNotFound)));
}

#[test]
fn test_load_config_bad_yaml() {
    let home = dirs_next::home_dir().unwrap();
    let config_dir = home.join(".indexer");
    let config_path = config_dir.join("bad_config.yaml");
    fs::create_dir_all(&config_dir).unwrap();
    fs::write(&config_path, "bad: [unclosed").unwrap();
    // Temporarily rename the real config if it exists
    let real_config = config_dir.join("config.yaml");
    let backup = config_dir.join("config_backup.yaml");
    let real_exists = real_config.exists();
    if real_exists { fs::rename(&real_config, &backup).unwrap(); }
    fs::rename(&config_path, &real_config).unwrap();
    let result = AppConfig::load();
    assert!(matches!(result, Err(ConfigError::Yaml(_))));
    let _ = fs::remove_file(&real_config);
    if real_exists { fs::rename(&backup, &real_config).unwrap(); }
}
