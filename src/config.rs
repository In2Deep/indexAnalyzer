//! configuration module for indexer
//! loads ~/.indexer/config.yaml using serde_yaml

use serde::Deserialize;
use std::fs;
use thiserror::Error;

use std::collections::HashMap;

/// Global defaults for the indexer
#[derive(Debug, Deserialize)]
pub struct GlobalDefaults {
    pub provider: String,
    pub db: String,
}

#[derive(Debug, Deserialize)]
pub struct ProviderConfig {
    pub api_key: String,
    pub model: String,
}

#[derive(Debug, Deserialize)]
pub struct VectorDbConfig {
    pub url: String,
    pub key_prefix: String,
}

#[derive(Debug, Deserialize)]
pub struct AppConfig {
    pub redis_url: Option<String>,
    pub log_level: Option<String>,
    pub global_defaults: Option<GlobalDefaults>,
    pub providers: Option<HashMap<String, ProviderConfig>>,
    pub vector_dbs: Option<HashMap<String, VectorDbConfig>>,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            redis_url: None,
            log_level: None,
            global_defaults: None,
            providers: None,
            vector_dbs: None,
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
                        global_defaults: yaml.global_defaults.or(default.global_defaults),
                        providers: yaml.providers.or(default.providers),
                        vector_dbs: yaml.vector_dbs.or(default.vector_dbs),
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
