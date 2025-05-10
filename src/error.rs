//! error module for indexer

use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
    #[error("yaml configuration error: {0}")]
    YamlConfig(#[from] serde_yaml::Error),
    #[error("redis error: {0}")]
    Redis(#[from] fred::error::Error),

}
