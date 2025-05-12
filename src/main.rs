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
use indexer::embedder::{Embedder, OpenAIEmbedder, HFEmbedder, MockEmbedder};
use indexer::vector_store::{VectorStore, RedisVectorStore};
// Import but don't use directly to avoid namespace conflicts
use indexer::vector_search;
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
        Commands::Vectorize { ref name, .. } => (format!("code_index:{}", name), args.command),
        Commands::VectorRecall { ref name, .. } => (format!("code_index:{}", name), args.command),
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
        Commands::Vectorize { name, path, provider, db, batch_size, dry_run, verbose } => {
            info!("Starting vectorize command for project: {}", name);
            
            // Create embedder based on provider or use MockEmbedder for testing
            let embedder = match provider.as_deref() {
                Some("openai") => {
                    match OpenAIEmbedder::new_from_env() {
                        Ok(e) => Box::new(e) as Box<dyn Embedder>,
                        Err(e) => {
                            eprintln!("Error creating OpenAI embedder: {}", e);
                            return Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, e)));
                        }
                    }
                },
                Some("hf") => {
                    match HFEmbedder::new_from_env() {
                        Ok(e) => Box::new(e) as Box<dyn Embedder>,
                        Err(e) => {
                            eprintln!("Error creating HuggingFace embedder: {}", e);
                            return Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, e)));
                        }
                    }
                },
                _ => Box::new(MockEmbedder::new()) as Box<dyn Embedder>
            };
            
            // Create vector store
            let redis_url = config.redis_url.as_deref().unwrap_or("redis://127.0.0.1/");
            let store = RedisVectorStore::new(redis_url, &key_prefix);
            
            // Call vectorize command directly without recreating CLI args
            // This avoids the namespace conflict between binary and library CLI types
            let project_path = PathBuf::from(path);
            if !project_path.exists() {
                let err_msg = format!("Project path does not exist: {}", path);
                eprintln!("{}", err_msg);
                return Err(Box::new(std::io::Error::new(std::io::ErrorKind::NotFound, err_msg)));
            }
            
            // Process the directory using the library functions directly
            let batch_size_val = batch_size.unwrap_or(10);
            
            // Use the library's process_directory function directly
            match indexer::vectorize::process_directory(&project_path, &*embedder, &store, batch_size_val, dry_run, verbose) {
                Ok(_) => {
                    if dry_run {
                        info!("Dry run completed successfully");
                    } else {
                        info!("Vectorization completed successfully");
                    }
                },
                Err(e) => {
                    eprintln!("Error during vectorization: {}", e);
                    return Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, e)));
                }
            }
        }
        Commands::VectorRecall { name, query, provider, db, top_k, json } => {
            info!("Starting vector recall for project: {}", name);
            
            // Create embedder based on provider or use MockEmbedder for testing
            let embedder = match provider.as_deref() {
                Some("openai") => {
                    match OpenAIEmbedder::new_from_env() {
                        Ok(e) => Box::new(e) as Box<dyn Embedder>,
                        Err(e) => {
                            eprintln!("Error creating OpenAI embedder: {}", e);
                            return Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, e)));
                        }
                    }
                },
                Some("hf") => {
                    match HFEmbedder::new_from_env() {
                        Ok(e) => Box::new(e) as Box<dyn Embedder>,
                        Err(e) => {
                            eprintln!("Error creating HuggingFace embedder: {}", e);
                            return Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, e)));
                        }
                    }
                },
                _ => Box::new(MockEmbedder::new()) as Box<dyn Embedder>
            };
            
            // Create vector store
            let redis_url = config.redis_url.as_deref().unwrap_or("redis://127.0.0.1/");
            let store = RedisVectorStore::new(redis_url, &key_prefix);
            
            // Generate embedding for query
            let query_embedding = embedder.embed(&query);
            
            // Set up search options
            let search_options = vector_search::SearchOptions {
                top_k: top_k.unwrap_or(5),
                entity_types: None,
                file_filter: None,
                min_score: Some(0.0),
            };
            
            // Perform search
            let results = vector_search::search_vectors(&store, &query_embedding, &search_options)
                .map_err(|e| Box::new(std::io::Error::new(std::io::ErrorKind::Other, e)))?;
            
            // Output results
            if json {
                let json_str = match serde_json::to_string_pretty(&results) {
                    Ok(s) => s,
                    Err(e) => {
                        eprintln!("Error serializing results to JSON: {}", e);
                        return Err(Box::new(e));
                    }
                };
                println!("{}", json_str);
            } else {
                println!("Search results for query: {}", query);
                for (i, result) in results.iter().enumerate() {
                    println!("{}: {} (score: {:.4})", i + 1, result.entity_id, result.score);
                    if let Ok(metadata) = store.get_entity_metadata(&result.entity_id) {
                        if let Some(file) = metadata.get("file") {
                            println!("   File: {}", file);
                        }
                        if let Some(entity_type) = metadata.get("type") {
                            println!("   Type: {}", entity_type);
                        }
                    }
                    println!();
                }
            }
        }
    }
    Ok(())
}
