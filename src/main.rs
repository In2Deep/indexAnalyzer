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
use code_indexer_rust::redis_ops::{create_redis_client, store_file_content, store_code_entities, clear_file_data, query_code_entity};
use fred::interfaces::SetsInterface;
use code_indexer_rust::file_processing::collect_python_files;
use code_indexer_rust::ast_parser::extract_code_info;
use clap::Parser;
use log::{error, info};
use std::path::PathBuf;
use std::process;
use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load config
    let config = AppConfig::load()?;
    let key_prefix = "code_index"; // Use a static key prefix
    // Setup logging
    setup_logging(&config)?;

    // Parse CLI
    let args = CliArgs::parse();

    // Connect to Redis
    let redis = create_redis_client(&config.redis_url.as_ref().unwrap()).await?;

    match args.command {
        Commands::Remember { path } => {
            let app_dir = PathBuf::from(path);
            let files = collect_python_files(&app_dir, None);
            for file in &files {
                let rel_path = file.strip_prefix(&app_dir).unwrap_or(file).to_string_lossy().to_string();
                let content = tokio::fs::read_to_string(file).await?;
                let meta = tokio::fs::metadata(file).await?;
                let size = meta.len() as usize;
                let mtime = meta.modified()?.elapsed().unwrap_or_default().as_secs() as i64;
                store_file_content(&redis, key_prefix, &rel_path, &content, size, mtime).await?;
                let entities = extract_code_info(file, &app_dir);
                store_code_entities(&redis, key_prefix, &entities).await?;
            }
            info!("Indexed {} files", files.len());
        }
        Commands::Refresh { files, project: _ } => {
            let app_dir = std::env::current_dir()?;
            let files: Vec<String> = files.split(',').map(|s| s.trim().to_string()).collect();
            let files = collect_python_files(&app_dir, Some(&files));
            for file in &files {
                let rel_path = file.strip_prefix(&app_dir).unwrap_or(file).to_string_lossy().to_string();
                let content = tokio::fs::read_to_string(file).await?;
                let meta = tokio::fs::metadata(file).await?;
                let size = meta.len() as usize;
                let mtime = meta.modified()?.elapsed().unwrap_or_default().as_secs() as i64;
                store_file_content(&redis, key_prefix, &rel_path, &content, size, mtime).await?;
                let entities = extract_code_info(file, &app_dir);
                store_code_entities(&redis, key_prefix, &entities).await?;
            }
            info!("Refreshed {} files", files.len());
        }
        Commands::Recall { entity_type, name, project: _ } => {
            let results = query_code_entity(&redis, key_prefix, &entity_type, name.as_deref()).await?;
            println!("{}", serde_json::to_string_pretty(&results)?);
        }
        Commands::Status { project: _ } => {
            let key = format!("{}:file_index", key_prefix);
            let files: Vec<String> = redis.smembers(&key).await.unwrap_or_default();
            println!("Indexed files: {}", files.len());
            for f in files {
                println!("- {}", f);
            }
        }
        Commands::Forget { project: _ } => {
            let files: Vec<String> = redis.smembers(format!("{}:file_index", key_prefix)).await.unwrap_or_default();
            clear_file_data(&redis, key_prefix, &files).await?;
            info!("Cleared all indexed data");
        }
    }
    Ok(())
}
