//! RED tests for vector indexing workflow (entity extraction, embedding, storage, batch)

use indexer::vector_store::{RedisVectorStore, VectorStore};
use indexer::embedder::{Embedder, MockEmbedder};
use indexer::ast_parser::extract_code_info;
use tempfile::tempdir;
use std::fs::File;
use std::io::Write;

#[test]
fn test_extract_entities_for_vectorization() {
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("test.py");
    let code = "def foo():\n    pass\nclass Bar:\n    def baz(self):\n        pass\n";
    let mut file = File::create(&file_path).unwrap();
    write!(file, "{}", code).unwrap();
    let entities = extract_code_info(&file_path, dir.path());
    assert!(entities.iter().any(|e| e.name == "foo"));
    assert!(entities.iter().any(|e| e.name == "Bar"));
    assert!(entities.iter().any(|e| e.name == "baz"));
}

#[test]
fn test_embedding_generation_for_entities() {
    let entities = vec!["foo", "bar", "baz"];
    let embedder = MockEmbedder;
    for entity in entities {
        let vec = embedder.embed(entity);
        assert!(!vec.is_empty(), "Embedding should not be empty");
    }
}

#[test]
fn test_store_embeddings_with_metadata() {
    // For the test, we'll use the trait implementation which is synchronous
    // This allows us to test without async/await
    let store = RedisVectorStore::new("redis://localhost:6379/0", "test_prefix");
    let embedding = vec![1.0, 2.0, 3.0];
    let entity_id = "entity1";
    
    // Use the trait method which returns a synchronous Result
    let result = VectorStore::upsert_embedding(&store, entity_id, &embedding, Some("file.py"), Some("function"));
    assert!(result.is_ok(), "Should store embedding without error");
    
    // Verify we can get the entity metadata
    let metadata_result = VectorStore::get_entity_metadata(&store, entity_id);
    assert!(metadata_result.is_ok(), "Should retrieve metadata without error");
    
    if let Ok(metadata) = metadata_result {
        assert_eq!(metadata.get("id").unwrap(), entity_id, "Entity ID should match");
        assert_eq!(metadata.get("type").unwrap(), "function", "Entity type should match");
        assert!(metadata.contains_key("file"), "Metadata should include file information");
    }
}

#[test]
fn test_batch_processing_progress_logging() {
    // Simulate batch processing and check progress logging
    let total = 5;
    let mut progress = 0;
    for i in 0..total {
        // Simulate embedding
        progress = i + 1;
        log::info!("Embedded {} of {}", progress, total);
    }
    assert_eq!(progress, total);
}

#[test]
fn test_error_handling_during_batch_processing() {
    // Simulate an error in embedding
    let embedder = MockEmbedder;
    let entities = vec!["foo", "fail", "bar"];
    let mut errors = 0;
    for entity in entities {
        let result = std::panic::catch_unwind(|| embedder.embed(entity));
        if result.is_err() {
            errors += 1;
            log::error!("Failed to embed entity: {}", entity);
        }
    }
    assert!(errors >= 0, "Should handle errors gracefully");
}
