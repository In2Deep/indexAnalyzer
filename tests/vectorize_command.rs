//! Tests for vectorize command implementation (TDD: Phase 2 - RED)

use indexer::cli::{CliArgs, Commands};
use indexer::config::AppConfig;
use indexer::embedder::MockEmbedder;
use indexer::vector_store::{RedisVectorStore, VectorStore};
use std::path::PathBuf;

#[tokio::test]
async fn test_vectorize_command_execution() {
    // This test verifies that the vectorize command properly extracts entities,
    // generates embeddings, and stores them in the vector store.
    
    // Setup mock components
    let mock_embedder = MockEmbedder::new();
    let store = RedisVectorStore::new("redis://localhost:6379/0", "test_prefix");
    let project_path = PathBuf::from("./test_data");
    
    // Create test CLI args
    let args = CliArgs {
        command: Commands::Vectorize {
            name: "test_project".to_string(),
            path: project_path.to_string_lossy().to_string(),
            provider: Some("mock".to_string()),
            db: Some("redis".to_string()),
            batch_size: Some(10),
            dry_run: false,
            verbose: true,
        },
    };
    
    // Call the vectorize command function
    let result = indexer::vectorize_command(&args, &mock_embedder, &store).await;
    
    // Verify the result
    assert!(result.is_ok(), "Vectorize command should succeed");
    
    // Verify that entities were extracted and stored
    let entity_ids = store.get_all_entity_ids().unwrap();
    assert!(!entity_ids.is_empty(), "Should have extracted and stored entities");
    
    // Verify logging occurred (would need to capture logs in a real test)
    // This is a placeholder assertion
    assert!(true, "Logging should have occurred");
}

#[tokio::test]
async fn test_vectorize_command_dry_run() {
    // This test verifies that the vectorize command in dry-run mode
    // extracts entities but does not store them.
    
    // Setup mock components
    let mock_embedder = MockEmbedder::new();
    let store = RedisVectorStore::new("redis://localhost:6379/0", "test_prefix");
    let project_path = PathBuf::from("./test_data");
    
    // Create test CLI args with dry_run = true
    let args = CliArgs {
        command: Commands::Vectorize {
            name: "test_project".to_string(),
            path: project_path.to_string_lossy().to_string(),
            provider: Some("mock".to_string()),
            db: Some("redis".to_string()),
            batch_size: Some(10),
            dry_run: true,
            verbose: false,
        },
    };
    
    // Call the vectorize command function
    let result = indexer::vectorize_command(&args, &mock_embedder, &store).await;
    
    // Verify the result
    assert!(result.is_ok(), "Vectorize command should succeed even in dry-run mode");
    
    // Verify that no entities were stored (since it's a dry run)
    let entity_ids = store.get_all_entity_ids().unwrap();
    assert!(entity_ids.is_empty(), "Should not have stored any entities in dry-run mode");
}

#[tokio::test]
async fn test_vectorize_command_with_config_fallback() {
    // This test verifies that the vectorize command falls back to
    // configuration values when CLI arguments are not provided.
    
    // Setup mock components
    let mock_embedder = MockEmbedder::new();
    let store = RedisVectorStore::new("redis://localhost:6379/0", "test_prefix");
    let project_path = PathBuf::from("./test_data");
    
    // Create test CLI args with minimal arguments
    let args = CliArgs {
        command: Commands::Vectorize {
            name: "test_project".to_string(),
            path: project_path.to_string_lossy().to_string(),
            provider: None,
            db: None,
            batch_size: None,
            dry_run: false,
            verbose: false,
        },
    };
    
    // Mock config would be loaded here in a real test
    // For now, we'll just verify the function handles missing arguments
    
    // Call the vectorize command function
    let result = indexer::vectorize_command(&args, &mock_embedder, &store).await;
    
    // Verify the result
    assert!(result.is_ok(), "Vectorize command should succeed with config fallbacks");
    
    // Verify that entities were extracted and stored
    let entity_ids = store.get_all_entity_ids().unwrap();
    assert!(!entity_ids.is_empty(), "Should have extracted and stored entities");
}
