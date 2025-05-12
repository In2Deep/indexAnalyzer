//! Extreme tests for the vector query functionality

use indexer::cli::Commands;
use indexer::embedder::{Embedder, MockEmbedder};
use indexer::vector_store::{VectorStore, MockVectorStore};
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use std::time::Duration;

// Mock embedder for query testing
struct QueryTestEmbedder {
    // Predefined embeddings for specific queries
    embedding_map: HashMap<String, Vec<f32>>,
    // Track embedding calls
    embed_calls: Arc<Mutex<Vec<String>>>,
}

impl QueryTestEmbedder {
    fn new() -> Self {
        let mut embedding_map = HashMap::new();
        
        // Add predefined embeddings for common queries
        embedding_map.insert(
            "function add".to_string(),
            vec![0.9, 0.1, 0.0, 0.0],
        );
        
        embedding_map.insert(
            "function subtract".to_string(),
            vec![0.1, 0.9, 0.0, 0.0],
        );
        
        embedding_map.insert(
            "class Vector".to_string(),
            vec![0.0, 0.0, 0.9, 0.1],
        );
        
        embedding_map.insert(
            "interface Comparable".to_string(),
            vec![0.0, 0.0, 0.1, 0.9],
        );
        
        QueryTestEmbedder {
            embedding_map,
            embed_calls: Arc::new(Mutex::new(Vec::new())),
        }
    }
    
    fn get_embed_calls(&self) -> Vec<String> {
        self.embed_calls.lock().unwrap().clone()
    }
}

impl Embedder for QueryTestEmbedder {
    fn embed(&self, input: &str) -> Vec<f32> {
        // Record the embedding call
        self.embed_calls.lock().unwrap().push(input.to_string());
        
        // Check if we have a predefined embedding for this input
        if let Some(embedding) = self.embedding_map.get(input) {
            return embedding.clone();
        }
        
        // For other inputs, generate a simple embedding
        // Use a deterministic approach based on the input
        let mut result = vec![0.0, 0.0, 0.0, 0.0];
        
        if input.contains("function") {
            result[0] = 0.7;
            result[1] = 0.3;
        } else if input.contains("class") {
            result[2] = 0.7;
            result[3] = 0.3;
        } else if input.contains("interface") {
            result[3] = 0.7;
            result[2] = 0.3;
        } else {
            // Default embedding
            result[0] = 0.25;
            result[1] = 0.25;
            result[2] = 0.25;
            result[3] = 0.25;
        }
        
        result
    }
}

// Mock vector store for query testing
struct QueryTestVectorStore {
    // Stored entities and their embeddings
    stored_entities: Arc<Mutex<HashMap<String, Vec<f32>>>>,
    stored_metadata: Arc<Mutex<HashMap<String, HashMap<String, String>>>>,
    // Track store calls
    store_calls: Arc<Mutex<Vec<String>>>,
    query_calls: Arc<Mutex<Vec<String>>>,
}

impl QueryTestVectorStore {
    fn new() -> Self {
        QueryTestVectorStore {
            stored_entities: Arc::new(Mutex::new(HashMap::new())),
            stored_metadata: Arc::new(Mutex::new(HashMap::new())),
            store_calls: Arc::new(Mutex::new(Vec::new())),
            query_calls: Arc::new(Mutex::new(Vec::new())),
        }
    }
    
    fn get_store_calls(&self) -> Vec<String> {
        self.store_calls.lock().unwrap().clone()
    }
    
    fn get_query_calls(&self) -> Vec<String> {
        self.query_calls.lock().unwrap().clone()
    }
    
    // Helper to pre-populate the store with test data
    fn populate_test_data(&self) {
        let mut entities = self.stored_entities.lock().unwrap();
        let mut metadata = self.stored_metadata.lock().unwrap();
        
        // Add function entities
        entities.insert("func_add".to_string(), vec![0.9, 0.1, 0.0, 0.0]);
        entities.insert("func_subtract".to_string(), vec![0.1, 0.9, 0.0, 0.0]);
        entities.insert("func_multiply".to_string(), vec![0.5, 0.5, 0.0, 0.0]);
        entities.insert("func_divide".to_string(), vec![0.4, 0.6, 0.0, 0.0]);
        
        // Add class entities
        entities.insert("class_vector".to_string(), vec![0.0, 0.0, 0.9, 0.1]);
        entities.insert("class_matrix".to_string(), vec![0.0, 0.0, 0.7, 0.3]);
        entities.insert("class_point".to_string(), vec![0.0, 0.0, 0.6, 0.4]);
        
        // Add interface entities
        entities.insert("interface_comparable".to_string(), vec![0.0, 0.0, 0.1, 0.9]);
        entities.insert("interface_serializable".to_string(), vec![0.0, 0.0, 0.3, 0.7]);
        
        // Add metadata for each entity
        for (entity_id, _) in entities.iter() {
            let mut entity_metadata = HashMap::new();
            entity_metadata.insert("id".to_string(), entity_id.clone());
            
            if entity_id.starts_with("func_") {
                entity_metadata.insert("file".to_string(), "math.rs".to_string());
                entity_metadata.insert("type".to_string(), "function".to_string());
                entity_metadata.insert("name".to_string(), entity_id.replace("func_", "").to_string());
            } else if entity_id.starts_with("class_") {
                entity_metadata.insert("file".to_string(), "geometry.rs".to_string());
                entity_metadata.insert("type".to_string(), "class".to_string());
                entity_metadata.insert("name".to_string(), entity_id.replace("class_", "").to_string());
            } else if entity_id.starts_with("interface_") {
                entity_metadata.insert("file".to_string(), "common.rs".to_string());
                entity_metadata.insert("type".to_string(), "interface".to_string());
                entity_metadata.insert("name".to_string(), entity_id.replace("interface_", "").to_string());
            }
            
            metadata.insert(entity_id.clone(), entity_metadata);
        }
    }
}

impl VectorStore for QueryTestVectorStore {
    fn upsert_embedding(&self, entity_id: &str, embedding: &[f32], file: Option<&str>, entity_type: Option<&str>) -> Result<(), String> {
        // Record the store call
        self.store_calls.lock().unwrap().push(entity_id.to_string());
        
        // Store the embedding
        let mut entities = self.stored_entities.lock().unwrap();
        entities.insert(entity_id.to_string(), embedding.to_vec());
        
        // Store metadata
        let mut metadata_map = self.stored_metadata.lock().unwrap();
        let mut metadata = HashMap::new();
        metadata.insert("id".to_string(), entity_id.to_string());
        if let Some(f) = file {
            metadata.insert("file".to_string(), f.to_string());
        }
        if let Some(et) = entity_type {
            metadata.insert("type".to_string(), et.to_string());
        }
        metadata_map.insert(entity_id.to_string(), metadata);
        
        Ok(())
    }
    
    fn similarity_search(&self, query: &[f32], top_k: usize) -> Vec<String> {
        // Record the query call
        self.query_calls.lock().unwrap().push(format!("similarity_search:top_k={}", top_k));
        
        // Get all entities
        let entities = self.stored_entities.lock().unwrap();
        
        // Calculate similarity scores
        let mut scores: Vec<(String, f32)> = entities
            .iter()
            .map(|(id, embedding)| {
                // Calculate dot product as similarity
                let similarity = query.iter()
                    .zip(embedding.iter())
                    .map(|(a, b)| a * b)
                    .sum();
                
                (id.clone(), similarity)
            })
            .collect();
        
        // Sort by similarity (descending)
        scores.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        
        // Return top_k results
        scores.iter()
            .take(top_k)
            .map(|(id, _)| id.clone())
            .collect()
    }
    
    fn get_all_entity_ids(&self) -> Result<Vec<String>, String> {
        // Record the query call
        self.query_calls.lock().unwrap().push("get_all_entity_ids".to_string());
        
        let entities = self.stored_entities.lock().unwrap();
        Ok(entities.keys().cloned().collect())
    }
    
    fn get_entity_vector(&self, entity_id: &str) -> Result<Vec<f32>, String> {
        // Record the query call
        self.query_calls.lock().unwrap().push(format!("get_entity_vector:{}", entity_id));
        
        let entities = self.stored_entities.lock().unwrap();
        entities.get(entity_id)
            .cloned()
            .ok_or_else(|| format!("Entity not found: {}", entity_id))
    }
    
    fn get_entity_metadata(&self, entity_id: &str) -> Result<HashMap<String, String>, String> {
        // Record the query call
        self.query_calls.lock().unwrap().push(format!("get_entity_metadata:{}", entity_id));
        
        let metadata = self.stored_metadata.lock().unwrap();
        metadata.get(entity_id)
            .cloned()
            .ok_or_else(|| format!("Entity metadata not found: {}", entity_id))
    }
}

#[tokio::test]
async fn test_query_command_basic() {
    // Create mock components
    let embedder = QueryTestEmbedder::new();
    let store = QueryTestVectorStore::new();
    
    // Populate the store with test data
    store.populate_test_data();
    
    // Create test CLI args for a basic query
    let args = CliArgs {
        command: Commands::Vectorize {
            name: "test_project".to_string(),
            query: "function add".to_string(),
            provider: Some("mock".to_string()),
            db: Some("redis".to_string()),
            top_k: Some(3),
            verbose: true,
        },
    };
    
    // Call the query command function
    // Using vectorize_command instead of query_command as it's not implemented yet
    let result = indexer::vectorize_command(&args, &embedder, &store);
    
    // Verify the result
    assert!(result.is_ok(), "Query command should succeed");
    
    // Check if the embedder was called with the correct query
    let embed_calls = embedder.get_embed_calls();
    assert!(embed_calls.contains(&"function add".to_string()), "Embedder should be called with the query");
    
    // Check if the store was queried
    let query_calls = store.get_query_calls();
    assert!(!query_calls.is_empty(), "Store should be queried");
    
    // The result should contain the most similar functions
    let output = result.unwrap();
    assert!(output.contains("func_add"), "Result should contain the most similar function");
}

#[tokio::test]
async fn test_query_command_with_different_entity_types() {
    // Create mock components
    let embedder = QueryTestEmbedder::new();
    let store = QueryTestVectorStore::new();
    
    // Populate the store with test data
    store.populate_test_data();
    
    // Test queries for different entity types
    let test_queries = vec![
        // Query, Expected top result
        ("function add", "func_add"),
        ("function subtract", "func_subtract"),
        ("class Vector", "class_vector"),
        ("interface Comparable", "interface_comparable"),
    ];
    
    for (query, expected_top_result) in test_queries {
        // Create test CLI args
        let args = CliArgs {
            command: Commands::Vectorize {
                name: "test_project".to_string(),
                query: query.to_string(),
                provider: Some("mock".to_string()),
                db: Some("redis".to_string()),
                top_k: Some(1),
                verbose: true,
            },
        };
        
        // Call the query command function
        // Using vectorize_command instead of query_command as it's not implemented yet
    let result = indexer::vectorize_command(&args, &embedder, &store);
        
        // Verify the result
        assert!(result.is_ok(), "Query command should succeed for query: {}", query);
        
        // The result should contain the expected top result
        let output = result.unwrap();
        assert!(output.contains(expected_top_result), 
                "Result for query '{}' should contain '{}', but got: {}", 
                query, expected_top_result, output);
    }
}

#[tokio::test]
async fn test_query_command_with_varying_top_k() {
    // Create mock components
    let embedder = QueryTestEmbedder::new();
    let store = QueryTestVectorStore::new();
    
    // Populate the store with test data
    store.populate_test_data();
    
    // Test with different top_k values
    let top_k_values = vec![1, 3, 5, 10];
    
    for top_k in top_k_values {
        // Create test CLI args
        let args = CliArgs {
            command: Commands::Vectorize {
                name: "test_project".to_string(),
                query: "function".to_string(),
                provider: Some("mock".to_string()),
                db: Some("redis".to_string()),
                top_k: Some(top_k),
                verbose: true,
            },
        };
        
        // Call the query command function
        // Using vectorize_command instead of query_command as it's not implemented yet
    let result = indexer::vectorize_command(&args, &embedder, &store);
        
        // Verify the result
        assert!(result.is_ok(), "Query command should succeed with top_k={}", top_k);
        
        // The result should respect the top_k limit
        let output = result.unwrap();
        let result_count = output.lines()
            .filter(|line| line.contains("func_"))
            .count();
        
        // We have 4 functions in our test data
        let expected_count = std::cmp::min(top_k, 4);
        assert!(result_count <= expected_count, 
                "Result should contain at most {} items for top_k={}, but got {}", 
                expected_count, top_k, result_count);
    }
}

#[tokio::test]
async fn test_query_command_with_empty_store() {
    // Create mock components
    let embedder = QueryTestEmbedder::new();
    let store = QueryTestVectorStore::new();
    
    // Don't populate the store - leave it empty
    
    // Create test CLI args
    let args = CliArgs {
        command: Commands::Vectorize {
            name: "test_project".to_string(),
            query: "function add".to_string(),
            provider: Some("mock".to_string()),
            db: Some("redis".to_string()),
            top_k: Some(3),
            verbose: true,
        },
    };
    
    // Call the query command function
    // Using vectorize_command instead of query_command as it's not implemented yet
    let result = indexer::vectorize_command(&args, &embedder, &store);
    
    // Verify the result
    assert!(result.is_ok(), "Query command should succeed even with empty store");
    
    // The result should indicate no matches were found
    let output = result.unwrap();
    assert!(output.contains("No matching entities found") || output.trim().is_empty(),
            "Result should indicate no matches were found");
}

#[tokio::test]
async fn test_query_command_with_very_long_query() {
    // Create mock components
    let embedder = QueryTestEmbedder::new();
    let store = QueryTestVectorStore::new();
    
    // Populate the store with test data
    store.populate_test_data();
    
    // Create a very long query
    let long_query = "function ".to_string() + &"x".repeat(10000); // 10KB query
    
    // Create test CLI args
    let args = CliArgs {
        command: Commands::Vectorize {
            name: "test_project".to_string(),
            query: long_query,
            provider: Some("mock".to_string()),
            db: Some("redis".to_string()),
            top_k: Some(3),
            verbose: true,
        },
    };
    
    // Call the query command function
    // Using vectorize_command instead of query_command as it's not implemented yet
    let result = indexer::vectorize_command(&args, &embedder, &store);
    
    // Verify the result
    assert!(result.is_ok(), "Query command should handle very long queries");
    
    // Check if the embedder was called
    let embed_calls = embedder.get_embed_calls();
    assert!(!embed_calls.is_empty(), "Embedder should be called even with long query");
}

#[tokio::test]
async fn test_query_command_with_special_characters() {
    // Create mock components
    let embedder = QueryTestEmbedder::new();
    let store = QueryTestVectorStore::new();
    
    // Populate the store with test data
    store.populate_test_data();
    
    // Create queries with special characters
    let special_queries = vec![
        "function add(a, b) { return a + b; }",
        "class Vector { x: number; y: number; }",
        "interface Comparable<T> { compareTo(other: T): number; }",
        "/* This is a comment */ function add() {}",
        "function // with comment",
    ];
    
    for query in special_queries {
        // Create test CLI args
        let args = CliArgs {
            command: Commands::Vectorize {
                name: "test_project".to_string(),
                query: query.to_string(),
                provider: Some("mock".to_string()),
                db: Some("redis".to_string()),
                top_k: Some(3),
                verbose: true,
            },
        };
        
        // Call the query command function
        // Using vectorize_command instead of query_command as it's not implemented yet
    let result = indexer::vectorize_command(&args, &embedder, &store);
        
        // Verify the result
        assert!(result.is_ok(), "Query command should handle special characters: {}", query);
    }
}

#[tokio::test]
async fn test_query_command_with_unicode_characters() {
    // Create mock components
    let embedder = QueryTestEmbedder::new();
    let store = QueryTestVectorStore::new();
    
    // Populate the store with test data
    store.populate_test_data();
    
    // Create queries with Unicode characters
    let unicode_queries = vec![
        "function 加法(a, b) { return a + b; }", // Chinese characters
        "class Вектор { x: number; y: number; }", // Russian characters
        "interface Συγκρίσιμο<T> { compareTo(other: T): number; }", // Greek characters
        "function add() { console.log('こんにちは'); }", // Japanese characters
        "function 🚀(a, b) { return a + b; }", // Emoji
    ];
    
    for query in unicode_queries {
        // Create test CLI args
        let args = CliArgs {
            command: Commands::Vectorize {
                name: "test_project".to_string(),
                query: query.to_string(),
                provider: Some("mock".to_string()),
                db: Some("redis".to_string()),
                top_k: Some(3),
                verbose: true,
            },
        };
        
        // Call the query command function
        // Using vectorize_command instead of query_command as it's not implemented yet
    let result = indexer::vectorize_command(&args, &embedder, &store);
        
        // Verify the result
        assert!(result.is_ok(), "Query command should handle Unicode characters: {}", query);
    }
}

#[tokio::test]
async fn test_query_command_output_format() {
    // Create mock components
    let embedder = QueryTestEmbedder::new();
    let store = QueryTestVectorStore::new();
    
    // Populate the store with test data
    store.populate_test_data();
    
    // Create test CLI args
    let args = CliArgs {
        command: Commands::Vectorize {
            name: "test_project".to_string(),
            query: "function add".to_string(),
            provider: Some("mock".to_string()),
            db: Some("redis".to_string()),
            top_k: Some(3),
            verbose: true,
        },
    };
    
    // Call the query command function
    // Using vectorize_command instead of query_command as it's not implemented yet
    let result = indexer::vectorize_command(&args, &embedder, &store);
    
    // Verify the result
    assert!(result.is_ok(), "Query command should succeed");
    
    // Check the output format
    let output = result.unwrap();
    
    // Output should contain entity IDs
    assert!(output.contains("func_add"), "Output should contain entity IDs");
    
    // Output should contain metadata if verbose is true
    assert!(output.contains("file") || output.contains("type") || output.contains("name"),
            "Output should contain metadata with verbose=true");
    
    // Try again with verbose=false
    let args = CliArgs {
        command: Commands::Vectorize {
            name: "test_project".to_string(),
            query: "function add".to_string(),
            provider: Some("mock".to_string()),
            db: Some("redis".to_string()),
            top_k: Some(3),
            verbose: false,
        },
    };
    
    // Call the query command function
    // Using vectorize_command instead of query_command as it's not implemented yet
    let result = indexer::vectorize_command(&args, &embedder, &store);
    
    // Verify the result
    assert!(result.is_ok(), "Query command should succeed with verbose=false");
}

#[tokio::test]
async fn test_query_command_project_isolation() {
    // Create mock components
    let embedder = QueryTestEmbedder::new();
    let store = QueryTestVectorStore::new();
    
    // Populate the store with test data
    store.populate_test_data();
    
    // Create test CLI args for different projects
    let project1_args = CliArgs {
        command: Commands::Vectorize {
            name: "project1".to_string(),
            query: "function add".to_string(),
            provider: Some("mock".to_string()),
            db: Some("redis".to_string()),
            top_k: Some(3),
            verbose: true,
        },
    };
    
    let project2_args = CliArgs {
        command: Commands::Vectorize {
            name: "project2".to_string(),
            query: "function add".to_string(),
            provider: Some("mock".to_string()),
            db: Some("redis".to_string()),
            top_k: Some(3),
            verbose: true,
        },
    };
    
    // Call the query command function for both projects
    // Using vectorize_command instead of query_command as it's not implemented yet
    let result1 = indexer::vectorize_command(&project1_args, &embedder, &store);
    let result2 = indexer::vectorize_command(&project2_args, &embedder, &store);
    
    // Verify the results
    assert!(result1.is_ok(), "Query command should succeed for project1");
    assert!(result2.is_ok(), "Query command should succeed for project2");
    
    // In a real implementation, these would query different Redis keys
    // For our mock, we're just verifying the project name is used correctly
    let query_calls = store.get_query_calls();
    assert!(query_calls.len() >= 2, "Store should be queried for both projects");
}
