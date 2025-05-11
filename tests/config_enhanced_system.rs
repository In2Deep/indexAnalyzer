//! RED test for enhanced config system loading (global defaults, providers, vector DBs, env API keys)

use indexer::config::AppConfig;
use std::fs;
use tempfile::tempdir;

#[test]
fn test_load_enhanced_config_system() {
    let temp = tempdir().unwrap();
    let home = temp.path();
    let config_dir = home.join(".indexer");
    fs::create_dir_all(&config_dir).unwrap();
    let config_path = config_dir.join("config.yaml");
    let yaml = r#"
redis_url: redis://localhost:6379/0
log_level: info
global_defaults:
  provider: openai
  db: redis
providers:
  openai:
    api_key: ENV_OPENAI_KEY
    model: text-embedding-ada-002
vector_dbs:
  redis:
    url: redis://localhost:6379/0
    key_prefix: code:myproject
"#;
    fs::write(&config_path, yaml).unwrap();
    std::env::set_var("HOME", home);
    let config = AppConfig::load().unwrap();
    assert_eq!(config.redis_url.as_deref(), Some("redis://localhost:6379/0"));
    assert_eq!(config.log_level.as_deref(), Some("info"));
    assert_eq!(config.global_defaults.as_ref().unwrap().provider, "openai");
    assert_eq!(config.global_defaults.as_ref().unwrap().db, "redis");
    assert_eq!(config.providers.as_ref().unwrap()["openai"].api_key, "ENV_OPENAI_KEY");
    assert_eq!(config.providers.as_ref().unwrap()["openai"].model, "text-embedding-ada-002");
    assert_eq!(config.vector_dbs.as_ref().unwrap()["redis"].url, "redis://localhost:6379/0");
    assert_eq!(config.vector_dbs.as_ref().unwrap()["redis"].key_prefix, "code:myproject");
}
