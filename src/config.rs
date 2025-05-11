//! configuration module for indexer
//! loads ~/.indexer/config.yaml using serde_yaml

use serde::Deserialize;

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
        
        let home = std::env::var("HOME").ok().map(std::path::PathBuf::from);
        let mut config_path = home.ok_or(ConfigError::HomeDirNotFound)?;
        config_path.push(".indexer");
        config_path.push("config.yaml");
        if config_path.exists() {
            let contents = fs::read_to_string(&config_path)?;
            match serde_yaml::from_str::<AppConfig>(&contents) {
                Ok(yaml) => {
                    let default = AppConfig::default();
                    Ok(AppConfig {
                        redis_url: yaml.redis_url.or(default.redis_url),
                        log_level: yaml.log_level.or(default.log_level),
                    })
                },
                Err(e) => Err(ConfigError::Yaml(e)),
            }
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
