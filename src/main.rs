//! main entrypoint for indexer
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
use crate::logging::setup_logging;
use indexer::redis_ops::{create_redis_client, store_file_content, store_code_entities, clear_file_data, query_code_entity};
use fred::interfaces::SetsInterface;
use indexer::file_processing::collect_python_files;
use indexer::ast_parser::extract_code_info;
use clap::Parser;
use log::info;
use std::path::PathBuf;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load config
    let config = AppConfig::load()?;
    // Parse CLI
    let args = CliArgs::parse();

    // Determine project name for Redis key prefix (from each command)
    let (key_prefix, cmd) = match args.command {
        Commands::Remember { ref name, .. } => (format!("code_index:{}", name), args.command),
        Commands::Refresh { ref name, .. } => (format!("code_index:{}", name), args.command),
        Commands::Recall { ref project_name, .. } => (format!("code_index:{}", project_name), args.command),
        Commands::Status { ref name } => (format!("code_index:{}", name), args.command),
        Commands::Forget { ref name } => (format!("code_index:{}", name), args.command),
    };
    // Setup logging
    setup_logging(&config)?;

    // Connect to Redis
    let redis = create_redis_client(config.redis_url.as_ref().unwrap()).await?;

    match cmd {
        Commands::Remember { name: _, path } => {
            let app_dir = PathBuf::from(path);
            let files = collect_python_files(&app_dir, None);
            for file in &files {
                let rel_path = file.strip_prefix(&app_dir).unwrap_or(file).to_string_lossy().to_string();
                let content = tokio::fs::read_to_string(file).await?;
                let meta = tokio::fs::metadata(file).await?;
                let size = meta.len() as usize;
                let mtime = meta.modified()?.elapsed().unwrap_or_default().as_secs() as i64;
                store_file_content(&redis, &key_prefix, &rel_path, &content, size, mtime).await?;
                let entities = extract_code_info(file, &app_dir);
                store_code_entities(&redis, &key_prefix, &entities).await?;
            }
            info!("Indexed {} files", files.len());
        }
        Commands::Refresh { name: _, files } => {
            let app_dir = std::env::current_dir()?;
            let files: Vec<String> = files.split(',').map(|s| s.trim().to_string()).collect();
            let files = collect_python_files(&app_dir, Some(&files));
            for file in &files {
                let rel_path = file.strip_prefix(&app_dir).unwrap_or(file).to_string_lossy().to_string();
                let content = tokio::fs::read_to_string(file).await?;
                let meta = tokio::fs::metadata(file).await?;
                let size = meta.len() as usize;
                let mtime = meta.modified()?.elapsed().unwrap_or_default().as_secs() as i64;
                store_file_content(&redis, &key_prefix, &rel_path, &content, size, mtime).await?;
                let entities = extract_code_info(file, &app_dir);
                store_code_entities(&redis, &key_prefix, &entities).await?;
            }
            info!("Refreshed {} files", files.len());
        }
        Commands::Recall { entity, show_lines, max: _max, project_name: _ } => {
            let entity_type = entity.as_deref().unwrap_or("");
            let results = query_code_entity(&redis, &key_prefix, entity_type, None).await?;
            if show_lines {
                for r in &results {
                    println!("{}: {}-{}", r.name, r.line_start, r.line_end);
                }
            } else {
                println!("{}", serde_json::to_string_pretty(&results)?);
            }
        }
        Commands::Status { name: _ } => {
            let key = format!("{}:file_index", key_prefix);
            let files: Vec<String> = redis.smembers(&key).await.unwrap_or_default();
            println!("Indexed files: {}", files.len());
            for f in files {
                println!("- {}", f);
            }
        }
        Commands::Forget { name: _ } => {
            let files: Vec<String> = redis.smembers(format!("{}:file_index", key_prefix)).await.unwrap_or_default();
            clear_file_data(&redis, &key_prefix, &files).await?;
            info!("Cleared all indexed data");
        }
    }
    Ok(())
}
