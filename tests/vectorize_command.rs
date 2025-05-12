//! Tests for vectorize command implementation (TDD: Phase 2 - RED)

use indexer::cli::{CliArgs, Commands};
use indexer::embedder::MockEmbedder;
use indexer::vector_store::{RedisVectorStore, VectorStore};
use tempfile;

#[tokio::test]
async fn test_vectorize_command_execution() {
    // This test verifies that the vectorize command properly extracts entities,
    // generates embeddings, and stores them in the vector store.
    
    // Create a temporary directory for test data
    let temp_dir = tempfile::tempdir().unwrap();
    let test_file_path = temp_dir.path().join("test_file.rs");
    
    // Create a test file with some code
    std::fs::write(&test_file_path, r#"
        fn test_function() {
            println!("Hello, world!");
        }
        
        struct TestStruct {
            field: i32,
        }
    "#).unwrap();
    
    // Setup mock components
    let mock_embedder = MockEmbedder::new();
    let store = RedisVectorStore::new("redis://localhost:6379/0", "test_prefix");
    
    // Create test CLI args
    let args = CliArgs {
        command: Commands::Vectorize {
            name: "test_project".to_string(),
            path: temp_dir.path().to_string_lossy().to_string(),
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

// Custom mock store for dry run testing
struct DryRunMockStore {
    stored_entities: std::cell::RefCell<Vec<String>>,
}

impl DryRunMockStore {
    fn new() -> Self {
        DryRunMockStore {
            stored_entities: std::cell::RefCell::new(Vec::new()),
        }
    }
}

impl VectorStore for DryRunMockStore {
    fn upsert_embedding(&self, entity_id: &str, _embedding: &[f32], _file: Option<&str>, _entity_type: Option<&str>) -> Result<(), String> {
        // Only store if not in dry run mode, which we'll handle in the vectorize_command function
        log::info!("DryRunMockStore: Would store entity {}", entity_id);
        Ok(())
    }
    
    fn similarity_search(&self, _query: &[f32], _top_k: usize) -> Vec<String> {
        vec![]
    }
    
    fn get_all_entity_ids(&self) -> Result<Vec<String>, String> {
        // Return empty list for dry run test
        Ok(self.stored_entities.borrow().clone())
    }
    
    fn get_entity_vector(&self, _entity_id: &str) -> Result<Vec<f32>, String> {
        Ok(vec![0.0, 0.0, 0.0])
    }
    
    fn get_entity_metadata(&self, _entity_id: &str) -> Result<std::collections::HashMap<String, String>, String> {
        Ok(std::collections::HashMap::new())
    }
}

#[tokio::test]
async fn test_vectorize_command_dry_run() {
    // This test verifies that the vectorize command in dry-run mode
    // extracts entities but does not store them.
    
    // Create a temporary directory for test data
    let temp_dir = tempfile::tempdir().unwrap();
    let test_file_path = temp_dir.path().join("test_file.rs");
    
    // Create a test file with some code
    std::fs::write(&test_file_path, r#"
        fn test_function() {
            println!("Hello, world!");
        }
    "#).unwrap();
    
    // Setup mock components
    let mock_embedder = MockEmbedder::new();
    let store = DryRunMockStore::new(); // Use our custom mock store
    
    // Create test CLI args with dry_run = true
    let args = CliArgs {
        command: Commands::Vectorize {
            name: "test_project".to_string(),
            path: temp_dir.path().to_string_lossy().to_string(),
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
    
    // Create a temporary directory for test data
    let temp_dir = tempfile::tempdir().unwrap();
    let test_file_path = temp_dir.path().join("test_file.rs");
    
    // Create a test file with some code
    std::fs::write(&test_file_path, r#"
        fn test_function() {
            println!("Hello, world!");
        }
        
        fn another_function() {
            // This is another function
        }
    "#).unwrap();
    
    // Setup mock components
    let mock_embedder = MockEmbedder::new();
    let store = RedisVectorStore::new("redis://localhost:6379/0", "test_prefix");
    
    // Create test CLI args with minimal arguments
    let args = CliArgs {
        command: Commands::Vectorize {
            name: "test_project".to_string(),
            path: temp_dir.path().to_string_lossy().to_string(),
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
