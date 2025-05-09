//! configuration module for code_indexer_rust
//! loads ~/.indexer/config.yaml using serde_yaml

use serde::Deserialize;
use dirs_next::home_dir;
use std::fs;
use thiserror::Error;

#[derive(Debug, Deserialize, Clone)]
pub struct AppConfig {
    pub redis_url: Option<String>,
    pub log_level: Option<String>,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            redis_url: Some("redis://127.0.0.1:6379/0".to_string()),
            log_level: Some("info".to_string()),
        }
    }
}

impl AppConfig {
    pub fn load() -> Result<Self, ConfigError> {
        let mut config_path = home_dir().ok_or(ConfigError::HomeDirNotFound)?;
        config_path.push(".indexer/config.yaml");
        if config_path.exists() {
            let contents = fs::read_to_string(&config_path)?;
            let yaml: AppConfig = serde_yaml::from_str(&contents)?;
            Ok(AppConfig {
                redis_url: yaml.redis_url.or_else(|| Some("redis://127.0.0.1:6379/0".to_string())),
                log_level: yaml.log_level.or_else(|| Some("info".to_string())),
            })
        } else {
            Ok(AppConfig::default())
        }
    }
}

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("could not determine home directory")] 
    HomeDirNotFound,
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
    #[error("yaml parse error: {0}")]
    Yaml(#[from] serde_yaml::Error),
}
