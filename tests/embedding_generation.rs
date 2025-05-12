//! RED tests for embedding generation for extracted entities
//! Tests error handling, batch processing, and different entity types

use indexer::extract_entities;
use indexer::embedder::{Embedder, OpenAIEmbedder, MockEmbedder};
use std::env;

#[test]
fn test_embedding_generation_with_missing_api_key() {
    // Temporarily clear the API key from the environment
    let key_backup = env::var("OPENAI_API_KEY").ok();
    env::remove_var("OPENAI_API_KEY");
    
    // This should return an error result
    let embedder_result = OpenAIEmbedder::new_from_env();
    assert!(embedder_result.is_err(), "Expected error with missing API key");
    
    // Restore the API key if it was present
    if let Some(key) = key_backup {
        env::set_var("OPENAI_API_KEY", key);
    }
}

#[test]
fn test_embedding_generation_for_different_entity_types() {
    let text = "class TestClass:\n    def test_method(self):\n        \"\"\"Test docstring\"\"\"\n        x = 10\n        pass\n";
    let entities = extract_entities(text);
    
    // We should have extracted multiple entity types (class, method, variable, docstring)
    assert!(entities.len() > 2, "Expected multiple entities, got: {:?}", entities);
    
    let embedder = MockEmbedder;
    
    // Generate embeddings for each entity type
    for entity in &entities {
        let embedding = embedder.embed(entity);
        
        // Mock embedder should return non-empty vectors
        assert!(!embedding.is_empty(), "Expected non-empty embedding for entity: {}", entity);
        
        // Different entity types should have different embeddings
        if entity.starts_with("class") {
            // This should fail because MockEmbedder returns [0.0, 1.0, 2.0] for all inputs
            assert_eq!(embedding, vec![1.0, 0.0, 0.0], "Expected class-specific embedding");
        } else if entity.starts_with("fn") {
            assert_eq!(embedding, vec![0.0, 1.0, 0.0], "Expected function-specific embedding");
        } else if entity.starts_with("var") {
            assert_eq!(embedding, vec![0.0, 0.0, 1.0], "Expected variable-specific embedding");
        } else if entity.starts_with("doc") {
            assert_eq!(embedding, vec![1.0, 1.0, 0.0], "Expected docstring-specific embedding");
        }
    }
}

#[test]
fn test_embedding_generation_batch_processing() {
    let texts = vec![
        "def func1(): pass",
        "def func2(): pass",
        "class Class1: pass",
    ];
    
    let embedder = MockEmbedder;
    let mut all_embeddings = Vec::new();
    
    // Process each text and collect all embeddings
    for text in texts {
        let entities = extract_entities(text);
        assert!(!entities.is_empty(), "Expected entities from text: {}", text);
        
        let embeddings: Vec<_> = entities.iter()
            .map(|e| embedder.embed(e))
            .collect();
        
        all_embeddings.extend(embeddings);
    }
    
    // Verify we have the expected number of embeddings
    assert!(all_embeddings.len() >= 3, "Expected at least 3 embeddings, got: {}", all_embeddings.len());
    
    // Verify all embeddings have the expected format
    for embedding in &all_embeddings {
        assert_eq!(embedding.len(), 3, "Expected embedding to have length 3");
    }
}

#[test]
fn test_embedding_generation_error_handling() {
    // Create a test case that should cause an error in embedding generation
    let entity = "error_trigger"; // MockEmbedder could be modified to return an error for this specific input
    
    // This test will need to be updated once we implement proper error handling in the embedder
    let embedder = MockEmbedder;
    let result = std::panic::catch_unwind(|| {
        embedder.embed(entity)
    });
    
    // For now, we're just verifying the test runs without panicking
    assert!(result.is_ok(), "Embedder should handle error cases gracefully");
}
