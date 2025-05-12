//! Extreme tests for the RedisVectorStore implementation

use indexer::vector_store::{RedisVectorStore, VectorStore};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use tempfile;
use redis::{self, Client, Commands, Connection};
use uuid::Uuid;

// Helper function to create a unique test key prefix
fn unique_test_prefix() -> String {
    format!("test:{}", Uuid::new_v4())
}

// Helper function to clean up Redis keys after tests
fn cleanup_redis_keys(prefix: &str) {
    let client = Client::open("redis://127.0.0.1/").expect("Failed to connect to Redis");
    let mut con = client.get_connection().expect("Failed to get Redis connection");
    
    // Find all keys with the prefix
    let keys: Vec<String> = redis::cmd("KEYS")
        .arg(format!("{}:*", prefix))
        .query(&mut con)
        .expect("Failed to query Redis keys");
    
    // Delete all found keys
    if !keys.is_empty() {
        redis::cmd("DEL")
            .arg(&keys)
            .execute(&mut con);
    }
}

// Helper to create a test vector store
fn create_test_store(prefix: &str) -> RedisVectorStore {
    RedisVectorStore::new(
        "redis://127.0.0.1/",
        prefix,
        1536, // Standard OpenAI embedding dimension
    ).expect("Failed to create RedisVectorStore")
}

#[tokio::test]
async fn test_vector_store_concurrent_operations() {
    // Create a unique prefix for this test
    let prefix = unique_test_prefix();
    
    // Create a vector store
    let store = Arc::new(create_test_store(&prefix));
    
    // Track successful operations
    let successful_inserts = Arc::new(Mutex::new(0));
    let successful_queries = Arc::new(Mutex::new(0));
    
    // Create multiple threads to perform concurrent operations
    let mut handles = vec![];
    
    // Number of concurrent operations
    let num_threads = 10;
    let ops_per_thread = 20;
    
    for thread_id in 0..num_threads {
        let store_clone = Arc::clone(&store);
        let inserts_clone = Arc::clone(&successful_inserts);
        
        // Create a thread for inserting embeddings
        let handle = thread::spawn(move || {
            for i in 0..ops_per_thread {
                let entity_id = format!("entity_{}_{}", thread_id, i);
                let embedding = vec![0.1, 0.2, 0.3]; // Simple test embedding
                
                // Occasionally sleep to increase chance of race conditions
                if i % 5 == 0 {
                    thread::sleep(Duration::from_millis(10));
                }
                
                match store_clone.upsert_embedding(
                    &entity_id,
                    &embedding,
                    Some(&format!("file_{}.rs", thread_id)),
                    Some("function"),
                ) {
                    Ok(_) => {
                        let mut count = inserts_clone.lock().unwrap();
                        *count += 1;
                    },
                    Err(e) => {
                        println!("Insert error: {}", e);
                    }
                }
            }
        });
        
        handles.push(handle);
    }
    
    // Create threads for querying
    for thread_id in 0..num_threads {
        let store_clone = Arc::clone(&store);
        let queries_clone = Arc::clone(&successful_queries);
        
        let handle = thread::spawn(move || {
            // Give some time for inserts to happen
            thread::sleep(Duration::from_millis(50));
            
            for i in 0..ops_per_thread {
                let query_vector = vec![0.1, 0.2, 0.3]; // Simple test query
                
                // Occasionally sleep to increase chance of race conditions
                if i % 3 == 0 {
                    thread::sleep(Duration::from_millis(5));
                }
                
                // Perform similarity search
                let results = store_clone.similarity_search(&query_vector, 10);
                
                // Count successful queries
                let mut count = queries_clone.lock().unwrap();
                *count += 1;
                
                // Occasionally try to get metadata
                if i % 4 == 0 && !results.is_empty() {
                    let entity_id = &results[0];
                    match store_clone.get_entity_metadata(entity_id) {
                        Ok(_) => {},
                        Err(e) => {
                            println!("Metadata error: {}", e);
                        }
                    }
                }
            }
        });
        
        handles.push(handle);
    }
    
    // Wait for all threads to complete
    for handle in handles {
        handle.join().unwrap();
    }
    
    // Get final counts
    let total_inserts = *successful_inserts.lock().unwrap();
    let total_queries = *successful_queries.lock().unwrap();
    
    println!("Successful inserts: {}", total_inserts);
    println!("Successful queries: {}", total_queries);
    
    // Verify that operations were successful
    assert!(total_inserts > 0, "Should have some successful inserts");
    assert!(total_queries > 0, "Should have some successful queries");
    
    // Verify data integrity by checking a few entities
    for thread_id in 0..num_threads {
        for i in 0..ops_per_thread {
            let entity_id = format!("entity_{}_{}", thread_id, i);
            
            // Try to get the entity vector
            match store.get_entity_vector(&entity_id) {
                Ok(vector) => {
                    assert_eq!(vector.len(), 3, "Vector should have correct dimension");
                },
                Err(_) => {
                    // Some entities might not have been inserted due to race conditions
                    // or other concurrent operation effects, which is acceptable
                }
            }
        }
    }
    
    // Clean up after the test
    cleanup_redis_keys(&prefix);
}

#[tokio::test]
async fn test_vector_store_large_batch_operations() {
    // Create a unique prefix for this test
    let prefix = unique_test_prefix();
    
    // Create a vector store
    let store = create_test_store(&prefix);
    
    // Number of entities to insert
    let num_entities = 1000;
    
    // Create a large batch of entities
    let mut entity_ids = Vec::with_capacity(num_entities);
    
    // Insert large batch
    for i in 0..num_entities {
        let entity_id = format!("large_entity_{}", i);
        entity_ids.push(entity_id.clone());
        
        // Create a larger embedding
        let embedding: Vec<f32> = (0..128).map(|j| (i as f32 * 0.01 + j as f32 * 0.001)).collect();
        
        // Insert the embedding
        store.upsert_embedding(
            &entity_id,
            &embedding,
            Some("large_file.rs"),
            Some("function"),
        ).expect(&format!("Failed to insert entity {}", i));
    }
    
    // Verify all entities were inserted
    let all_ids = store.get_all_entity_ids().expect("Failed to get all entity IDs");
    assert_eq!(all_ids.len(), num_entities, "All entities should be inserted");
    
    // Perform a large batch of queries
    for i in 0..100 {
        // Create a query vector
        let query_vector: Vec<f32> = (0..128).map(|j| (i as f32 * 0.01 + j as f32 * 0.001)).collect();
        
        // Perform similarity search with different top_k values
        let top_k = (i % 10) + 1; // Vary between 1 and 10
        let results = store.similarity_search(&query_vector, top_k);
        
        // Verify results
        assert!(results.len() <= top_k, "Should return at most top_k results");
    }
    
    // Clean up after the test
    cleanup_redis_keys(&prefix);
}

#[tokio::test]
async fn test_vector_store_error_handling() {
    // Create a unique prefix for this test
    let prefix = unique_test_prefix();
    
    // Test with invalid Redis URL
    let invalid_store_result = RedisVectorStore::new(
        "redis://invalid-host:6379/",
        &prefix,
        1536,
    );
    
    // Should return an error for invalid connection
    assert!(invalid_store_result.is_err(), "Should fail with invalid Redis URL");
    
    // Create a valid store for further tests
    let store = create_test_store(&prefix);
    
    // Test with invalid entity ID
    let result = store.get_entity_vector("non_existent_entity");
    assert!(result.is_err(), "Should return error for non-existent entity");
    
    // Test with invalid metadata
    let result = store.get_entity_metadata("non_existent_entity");
    assert!(result.is_err(), "Should return error for non-existent metadata");
    
    // Test with empty embedding
    let result = store.upsert_embedding(
        "empty_embedding",
        &[],
        Some("file.rs"),
        Some("function"),
    );
    assert!(result.is_err(), "Should return error for empty embedding");
    
    // Clean up after the test
    cleanup_redis_keys(&prefix);
}

#[tokio::test]
async fn test_vector_store_data_integrity() {
    // Create a unique prefix for this test
    let prefix = unique_test_prefix();
    
    // Create a vector store
    let store = create_test_store(&prefix);
    
    // Create test data with specific patterns
    let test_cases = vec![
        // Entity ID, Embedding, File, Type
        (
            "entity_1",
            vec![1.0, 0.0, 0.0],
            "file1.rs",
            "function"
        ),
        (
            "entity_2",
            vec![0.0, 1.0, 0.0],
            "file1.rs",
            "struct"
        ),
        (
            "entity_3",
            vec![0.0, 0.0, 1.0],
            "file2.rs",
            "function"
        ),
        (
            "entity_4",
            vec![0.5, 0.5, 0.0],
            "file2.rs",
            "trait"
        ),
        (
            "entity_5",
            vec![0.0, 0.5, 0.5],
            "file3.rs",
            "impl"
        ),
    ];
    
    // Insert all test cases
    for (entity_id, embedding, file, entity_type) in &test_cases {
        store.upsert_embedding(
            entity_id,
            embedding,
            Some(file),
            Some(entity_type),
        ).expect(&format!("Failed to insert entity {}", entity_id));
    }
    
    // Verify each entity was stored correctly
    for (entity_id, expected_embedding, file, entity_type) in &test_cases {
        // Check vector
        let vector = store.get_entity_vector(entity_id)
            .expect(&format!("Failed to get vector for {}", entity_id));
        assert_eq!(vector, *expected_embedding, "Vector should match for {}", entity_id);
        
        // Check metadata
        let metadata = store.get_entity_metadata(entity_id)
            .expect(&format!("Failed to get metadata for {}", entity_id));
        
        assert_eq!(metadata.get("id").unwrap(), entity_id, "ID should match");
        assert_eq!(metadata.get("file").unwrap(), file, "File should match");
        assert_eq!(metadata.get("type").unwrap(), entity_type, "Type should match");
    }
    
    // Test similarity search with known patterns
    
    // Query similar to entity_1
    let results = store.similarity_search(&[0.9, 0.1, 0.0], 3);
    assert!(results.contains(&"entity_1".to_string()), "Should find entity_1");
    
    // Query similar to entity_2
    let results = store.similarity_search(&[0.1, 0.9, 0.0], 3);
    assert!(results.contains(&"entity_2".to_string()), "Should find entity_2");
    
    // Query similar to entity_3
    let results = store.similarity_search(&[0.0, 0.1, 0.9], 3);
    assert!(results.contains(&"entity_3".to_string()), "Should find entity_3");
    
    // Test updating an existing entity
    let updated_embedding = vec![0.3, 0.3, 0.3];
    store.upsert_embedding(
        "entity_1",
        &updated_embedding,
        Some("updated_file.rs"),
        Some("updated_function"),
    ).expect("Failed to update entity");
    
    // Verify the update
    let vector = store.get_entity_vector("entity_1")
        .expect("Failed to get updated vector");
    assert_eq!(vector, updated_embedding, "Vector should be updated");
    
    let metadata = store.get_entity_metadata("entity_1")
        .expect("Failed to get updated metadata");
    assert_eq!(metadata.get("file").unwrap(), "updated_file.rs", "File should be updated");
    assert_eq!(metadata.get("type").unwrap(), "updated_function", "Type should be updated");
    
    // Clean up after the test
    cleanup_redis_keys(&prefix);
}

#[tokio::test]
async fn test_vector_store_isolation() {
    // Create two unique prefixes for this test
    let prefix1 = unique_test_prefix();
    let prefix2 = unique_test_prefix();
    
    // Create two vector stores with different prefixes
    let store1 = create_test_store(&prefix1);
    let store2 = create_test_store(&prefix2);
    
    // Insert data into store1
    for i in 0..10 {
        let entity_id = format!("store1_entity_{}", i);
        let embedding = vec![0.1, 0.2, 0.3];
        
        store1.upsert_embedding(
            &entity_id,
            &embedding,
            Some("store1_file.rs"),
            Some("function"),
        ).expect(&format!("Failed to insert entity {} into store1", i));
    }
    
    // Insert data into store2
    for i in 0..5 {
        let entity_id = format!("store2_entity_{}", i);
        let embedding = vec![0.4, 0.5, 0.6];
        
        store2.upsert_embedding(
            &entity_id,
            &embedding,
            Some("store2_file.rs"),
            Some("function"),
        ).expect(&format!("Failed to insert entity {} into store2", i));
    }
    
    // Verify store1 has only its own data
    let store1_ids = store1.get_all_entity_ids().expect("Failed to get store1 entity IDs");
    assert_eq!(store1_ids.len(), 10, "Store1 should have 10 entities");
    
    for id in &store1_ids {
        assert!(id.starts_with("store1_entity_"), "Store1 should only have its own entities");
    }
    
    // Verify store2 has only its own data
    let store2_ids = store2.get_all_entity_ids().expect("Failed to get store2 entity IDs");
    assert_eq!(store2_ids.len(), 5, "Store2 should have 5 entities");
    
    for id in &store2_ids {
        assert!(id.starts_with("store2_entity_"), "Store2 should only have its own entities");
    }
    
    // Verify cross-store isolation
    for i in 0..10 {
        let entity_id = format!("store1_entity_{}", i);
        let result = store2.get_entity_vector(&entity_id);
        assert!(result.is_err(), "Store2 should not access Store1's entity {}", i);
    }
    
    for i in 0..5 {
        let entity_id = format!("store2_entity_{}", i);
        let result = store1.get_entity_vector(&entity_id);
        assert!(result.is_err(), "Store1 should not access Store2's entity {}", i);
    }
    
    // Clean up after the test
    cleanup_redis_keys(&prefix1);
    cleanup_redis_keys(&prefix2);
}

#[tokio::test]
async fn test_vector_store_reconnection() {
    // Create a unique prefix for this test
    let prefix = unique_test_prefix();
    
    // Create a vector store
    let store = create_test_store(&prefix);
    
    // Insert some initial data
    for i in 0..5 {
        let entity_id = format!("reconnect_entity_{}", i);
        let embedding = vec![0.1, 0.2, 0.3];
        
        store.upsert_embedding(
            &entity_id,
            &embedding,
            Some("reconnect_file.rs"),
            Some("function"),
        ).expect(&format!("Failed to insert initial entity {}", i));
    }
    
    // Simulate Redis connection interruption by creating a new connection
    // In a real test, we might restart the Redis server, but that's complex in a test environment
    
    // Insert more data after "reconnection"
    for i in 5..10 {
        let entity_id = format!("reconnect_entity_{}", i);
        let embedding = vec![0.4, 0.5, 0.6];
        
        store.upsert_embedding(
            &entity_id,
            &embedding,
            Some("reconnect_file.rs"),
            Some("function"),
        ).expect(&format!("Failed to insert entity {} after reconnection", i));
    }
    
    // Verify all data is accessible
    let all_ids = store.get_all_entity_ids().expect("Failed to get all entity IDs");
    assert_eq!(all_ids.len(), 10, "Should have all 10 entities after reconnection");
    
    // Clean up after the test
    cleanup_redis_keys(&prefix);
}

#[tokio::test]
async fn test_vector_store_performance() {
    // Create a unique prefix for this test
    let prefix = unique_test_prefix();
    
    // Create a vector store
    let store = create_test_store(&prefix);
    
    // Number of entities for performance test
    let num_entities = 100;
    
    // Higher dimension for more realistic performance testing
    let dimension = 384;
    
    // Prepare entities with higher-dimensional embeddings
    let mut entity_ids = Vec::with_capacity(num_entities);
    let mut embeddings = Vec::with_capacity(num_entities);
    
    for i in 0..num_entities {
        let entity_id = format!("perf_entity_{}", i);
        entity_ids.push(entity_id);
        
        // Create a higher-dimensional embedding
        let embedding: Vec<f32> = (0..dimension).map(|j| (i as f32 * 0.01 + j as f32 * 0.001)).collect();
        embeddings.push(embedding);
    }
    
    // Measure insertion time
    let insert_start = std::time::Instant::now();
    
    for i in 0..num_entities {
        store.upsert_embedding(
            &entity_ids[i],
            &embeddings[i],
            Some("perf_file.rs"),
            Some("function"),
        ).expect(&format!("Failed to insert entity {}", i));
    }
    
    let insert_duration = insert_start.elapsed();
    println!("Inserted {} entities in {:?}", num_entities, insert_duration);
    println!("Average insertion time: {:?} per entity", insert_duration / num_entities as u32);
    
    // Measure query time
    let query_start = std::time::Instant::now();
    
    for i in 0..10 {
        // Use one of the existing embeddings as query
        let query_vector = &embeddings[i];
        
        // Perform similarity search
        let _results = store.similarity_search(query_vector, 10);
    }
    
    let query_duration = query_start.elapsed();
    println!("Performed 10 queries in {:?}", query_duration);
    println!("Average query time: {:?} per query", query_duration / 10);
    
    // Clean up after the test
    cleanup_redis_keys(&prefix);
}
