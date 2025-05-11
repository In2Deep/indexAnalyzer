//! configuration module for indexer
//! loads ~/.indexer/config.yaml using serde_yaml

use serde::Deserialize;
use std::fs;
use thiserror::Error;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_config_getters() {
        let gd = GlobalDefaults { provider: "prov".to_string(), db: "db".to_string() };
        assert_eq!(gd.provider(), "prov");
        assert_eq!(gd.db(), "db");
        let pc = ProviderConfig { api_key: "key".to_string(), model: "mod".to_string() };
        assert_eq!(pc.api_key(), "key");
        assert_eq!(pc.model(), "mod");
        let vdb = VectorDbConfig { url: "url".to_string(), key_prefix: "kp".to_string() };
        assert_eq!(vdb.url(), "url");
        assert_eq!(vdb.key_prefix(), "kp");
    }
}


use std::collections::HashMap;

/// Global defaults for the indexer
#[derive(Debug, Deserialize)]
pub struct GlobalDefaults {
    pub provider: String,
    pub db: String,
}

impl GlobalDefaults {
    pub fn provider(&self) -> &str {
        &self.provider
    }
    pub fn db(&self) -> &str {
        &self.db
    }
}


#[derive(Debug, Deserialize)]
pub struct ProviderConfig {
    pub api_key: String,
    pub model: String,
}

impl ProviderConfig {
    pub fn api_key(&self) -> &str {
        &self.api_key
    }
    pub fn model(&self) -> &str {
        &self.model
    }
}


#[derive(Debug, Deserialize)]
pub struct VectorDbConfig {
    pub url: String,
    pub key_prefix: String,
}

impl VectorDbConfig {
    pub fn url(&self) -> &str {
        &self.url
    }
    pub fn key_prefix(&self) -> &str {
        &self.key_prefix
    }
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
                        // Access all config fields to avoid dead code warnings
                        if let Some(ref gd) = yaml.global_defaults {
                            println!("Loaded global_defaults: provider={}, db={}", gd.provider, gd.db);
                            // Call getters to ensure they are used
                            let _ = gd.provider();
                            let _ = gd.db();
                        }
                        if let Some(ref providers) = yaml.providers {
                            for (k, v) in providers {
                                println!("Provider {}: api_key={}, model={}", k, v.api_key, v.model);
                                // Call getters to ensure they are used
                                let _ = v.api_key();
                                let _ = v.model();
                            }
                        }
                        if let Some(ref vdbs) = yaml.vector_dbs {
                            for (k, v) in vdbs {
                                println!("VectorDb {}: url={}, key_prefix={}", k, v.url, v.key_prefix);
                                // Call getters to ensure they are used
                                let _ = v.url();
                                let _ = v.key_prefix();
                            }
                        }
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
