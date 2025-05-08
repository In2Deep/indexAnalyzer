//! main entrypoint for code_indexer_rust
//! - loads configuration
//! - sets up logging
//! - parses cli
//! - dispatches to async runtime

mod config;
mod error;
mod cli;
mod logging;

use crate::config::AppConfig;
use crate::cli::{CliArgs, Commands};
use crate::error::AppError;
use crate::logging::setup_logging;
use crate::redis_ops::{create_redis_client, store_file_content, store_code_entities, clear_file_data, query_code_entity};
use crate::file_processing::collect_python_files;
use crate::ast_parser::extract_code_info;
use clap::Parser;
use log::{error, info};
use std::path::PathBuf;
use std::process;
use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load config
    let config = AppConfig::load()?;
    // Setup logging
    crate::logging::setup_logging(&config)?;
        eprintln!("logging error: {e}");
        process::exit(1);
    }

    // Parse CLI
    let cli = CliArgs::parse();

    // Dispatch to async logic (to be implemented)
    // match cli.command { ... }
}
