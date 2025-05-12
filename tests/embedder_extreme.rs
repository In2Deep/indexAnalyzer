//! Extreme tests for the embedder implementations

use indexer::embedder::{Embedder, HFEmbedder, OpenAIEmbedder};
use indexer::vector_store::VectorStore;
use std::sync::{Arc, Mutex};
use tempfile;
use uuid::Uuid;
use fred::prelude::*;
use std::time::Duration;
use log::{info, warn, error};

// Mock embedder for testing without API keys
struct MockEmbedder {
    // Track embedding calls
    embed_calls: Arc<Mutex<Vec<String>>>,
    // Control dimension of returned embeddings
    dimension: usize,
    // Track failures
    failures: Arc<Mutex<Vec<String>>>,
    // Simulate rate limiting
    rate_limit_after: usize,
    call_count: Arc<Mutex<usize>>,
    // Simulate latency
    latency_ms: u64,
}

impl MockEmbedder {
    fn new(dimension: usize, rate_limit_after: usize, latency_ms: u64) -> Self {
        MockEmbedder {
            embed_calls: Arc::new(Mutex::new(Vec::new())),
            dimension,
            failures: Arc::new(Mutex::new(Vec::new())),
            rate_limit_after,
            call_count: Arc::new(Mutex::new(0)),
            latency_ms,
        }
    }
    
    fn get_embed_calls(&self) -> Vec<String> {
        self.embed_calls.lock().unwrap().clone()
    }
    
    fn get_failures(&self) -> Vec<String> {
        self.failures.lock().unwrap().clone()
    }
}

impl Embedder for MockEmbedder {
    fn embed(&self, input: &str) -> Vec<f32> {
        // Record the embedding call
        self.embed_calls.lock().unwrap().push(input.to_string());
        
        // Increment call count
        let mut count = self.call_count.lock().unwrap();
        *count += 1;
        
        // Check if we should simulate rate limiting
        if *count > self.rate_limit_after {
            // Record failure
            self.failures.lock().unwrap().push(format!("Rate limit exceeded for {}", input));
            
            // Simulate longer delay for rate limiting
            std::thread::sleep(Duration::from_millis(self.latency_ms * 5));
            
            // Return empty embedding to simulate failure
            return vec![0.0; self.dimension];
        }
        
        // Simulate API latency
        if self.latency_ms > 0 {
            std::thread::sleep(Duration::from_millis(self.latency_ms));
        }
        
        // Generate deterministic embedding based on input
        let mut embedding = Vec::with_capacity(self.dimension);
        let input_hash = input.bytes().fold(0u64, |acc, b| acc.wrapping_add(b as u64));
        
        for i in 0..self.dimension {
            let value = ((input_hash + i as u64) % 1000) as f32 / 1000.0;
            embedding.push(value);
        }
        
        embedding
    }
}

// Helper function to check if embeddings are valid
fn is_valid_embedding(embedding: &[f32], expected_dim: usize) -> bool {
    if embedding.len() != expected_dim {
        return false;
    }
    
    // Check if embedding contains non-zero values
    let non_zero = embedding.iter().any(|&x| x != 0.0);
    
    // Check if embedding is normalized (approximately)
    let sum_squared: f32 = embedding.iter().map(|x| x * x).sum();
    let magnitude = sum_squared.sqrt();
    let is_normalized = (magnitude - 1.0).abs() < 0.01;
    
    non_zero && is_normalized
}

#[tokio::test]
async fn test_mock_embedder_basic() {
    // Create a mock embedder
    let embedder = MockEmbedder::new(
        1536, // OpenAI-like dimension
        100,  // Rate limit after 100 calls
        10,   // 10ms latency
    );
    
    // Test with simple input
    let input = "function test() { console.log('hello'); }";
    let embedding = embedder.embed(input);
    
    // Verify embedding dimension
    assert_eq!(embedding.len(), 1536, "Embedding should have correct dimension");
    
    // Verify call was recorded
    let calls = embedder.get_embed_calls();
    assert_eq!(calls.len(), 1, "Should record one embedding call");
    assert_eq!(calls[0], input, "Should record the correct input");
}

#[tokio::test]
async fn test_mock_embedder_rate_limiting() {
    // Create a mock embedder with low rate limit
    let embedder = MockEmbedder::new(
        1536, // OpenAI-like dimension
        5,    // Rate limit after 5 calls
        10,   // 10ms latency
    );
    
    // Make several calls to trigger rate limiting
    for i in 0..10 {
        let input = format!("function test{}() {{}}", i);
        embedder.embed(&input);
    }
    
    // Verify failures were recorded
    let failures = embedder.get_failures();
    assert!(!failures.is_empty(), "Should record rate limit failures");
    assert_eq!(failures.len(), 5, "Should have 5 rate-limited calls");
}

#[tokio::test]
async fn test_embedder_with_extreme_inputs() {
    // Create a mock embedder
    let embedder = MockEmbedder::new(
        1536, // OpenAI-like dimension
        100,  // Rate limit after 100 calls
        0,    // No latency
    );
    
    // Test with empty input
    let empty_embedding = embedder.embed("");
    assert_eq!(empty_embedding.len(), 1536, "Should handle empty input");
    
    // Test with very long input
    let long_input = "x".repeat(100000); // 100KB string
    let long_embedding = embedder.embed(&long_input);
    assert_eq!(long_embedding.len(), 1536, "Should handle very long input");
    
    // Test with special characters
    let special_chars = "!@#$%^&*()_+{}|:<>?~`-=[]\\;',./";
    let special_embedding = embedder.embed(special_chars);
    assert_eq!(special_embedding.len(), 1536, "Should handle special characters");
    
    // Test with Unicode characters
    let unicode = "こんにちは世界! Привет, мир! 🚀 🌍 👨‍💻";
    let unicode_embedding = embedder.embed(unicode);
    assert_eq!(unicode_embedding.len(), 1536, "Should handle Unicode characters");
    
    // Test with code containing comments
    let code_with_comments = r#"
    // This is a comment
    function test() {
        /* This is a
           multi-line comment */
        return "test";
    }
    "#;
    let comment_embedding = embedder.embed(code_with_comments);
    assert_eq!(comment_embedding.len(), 1536, "Should handle code with comments");
}

#[tokio::test]
async fn test_embedder_concurrent_operations() {
    // Create a mock embedder
    let embedder = Arc::new(MockEmbedder::new(
        1536, // OpenAI-like dimension
        1000, // High rate limit
        50,   // 50ms latency to make concurrency effects more visible
    ));
    
    // Track successful operations
    let successful_embeds = Arc::new(Mutex::new(0));
    
    // Create multiple threads to perform concurrent operations
    let mut handles = vec![];
    
    // Number of concurrent operations
    let num_threads = 10;
    let ops_per_thread = 20;
    
    for thread_id in 0..num_threads {
        let embedder_clone = Arc::clone(&embedder);
        let success_clone = Arc::clone(&successful_embeds);
        
        // Create a thread for generating embeddings
        let handle = std::thread::spawn(move || {
            for i in 0..ops_per_thread {
                let input = format!("function thread_{}_op_{}() {{}}", thread_id, i);
                
                // Generate embedding
                let embedding = embedder_clone.embed(&input);
                
                // Verify embedding
                if embedding.len() == 1536 {
                    let mut count = success_clone.lock().unwrap();
                    *count += 1;
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
    let total_successes = *successful_embeds.lock().unwrap();
    let total_calls = embedder.get_embed_calls().len();
    
    println!("Successful embeds: {}", total_successes);
    println!("Total embedding calls: {}", total_calls);
    
    // Verify that operations were successful
    assert_eq!(total_successes, num_threads * ops_per_thread, "All operations should succeed");
    assert_eq!(total_calls, num_threads * ops_per_thread, "All calls should be recorded");
}

// Only run these tests if API keys are available
#[tokio::test]
#[ignore] // Ignore by default to avoid API costs
async fn test_openai_embedder_if_key_available() {
    // Check if OpenAI API key is available
    let api_key = std::env::var("OPENAI_API_KEY").ok();
    
    if api_key.is_none() {
        println!("Skipping OpenAI embedder test: No API key available");
        return;
    }
    
    // Create OpenAI embedder
    let embedder = OpenAIEmbedder::new(&api_key.unwrap(), "text-embedding-ada-002")
        .expect("Failed to create OpenAI embedder");
    
    // Test with simple input
    let input = "function test() { console.log('hello'); }";
    let embedding = embedder.embed(input);
    
    // Verify embedding
    assert!(is_valid_embedding(&embedding, 1536), "Should return valid embedding");
    
    // Test with empty input
    let empty_embedding = embedder.embed("");
    assert!(is_valid_embedding(&empty_embedding, 1536), "Should handle empty input");
    
    // Test with longer input
    let long_input = "x".repeat(1000); // 1KB string
    let long_embedding = embedder.embed(&long_input);
    assert!(is_valid_embedding(&long_embedding, 1536), "Should handle longer input");
}

#[tokio::test]
#[ignore] // Ignore by default to avoid API costs
async fn test_hf_embedder_if_key_available() {
    // Check if HuggingFace API key is available
    let api_key = std::env::var("HF_API_KEY").ok();
    
    if api_key.is_none() {
        println!("Skipping HuggingFace embedder test: No API key available");
        return;
    }
    
    // Create HuggingFace embedder
    let embedder = HFEmbedder::new(
        &api_key.unwrap(),
        "sentence-transformers/all-MiniLM-L6-v2", // Smaller model for testing
    ).expect("Failed to create HuggingFace embedder");
    
    // Test with simple input
    let input = "function test() { console.log('hello'); }";
    let embedding = embedder.embed(input);
    
    // Expected dimension for this model
    let expected_dim = 384;
    
    // Verify embedding
    assert!(is_valid_embedding(&embedding, expected_dim), "Should return valid embedding");
    
    // Test with empty input
    let empty_embedding = embedder.embed("");
    assert!(is_valid_embedding(&empty_embedding, expected_dim), "Should handle empty input");
    
    // Test with longer input
    let long_input = "x".repeat(1000); // 1KB string
    let long_embedding = embedder.embed(&long_input);
    assert!(is_valid_embedding(&long_embedding, expected_dim), "Should handle longer input");
}

#[tokio::test]
async fn test_embedder_consistency() {
    // Create a mock embedder
    let embedder = MockEmbedder::new(
        1536, // OpenAI-like dimension
        100,  // Rate limit after 100 calls
        0,    // No latency
    );
    
    // Test that the same input produces the same embedding
    let input = "function test() { return 42; }";
    
    let embedding1 = embedder.embed(input);
    let embedding2 = embedder.embed(input);
    
    // Verify embeddings are identical
    assert_eq!(embedding1, embedding2, "Same input should produce same embedding");
    
    // Test that different inputs produce different embeddings
    let input2 = "function test() { return 43; }";
    let embedding3 = embedder.embed(input2);
    
    // Verify embeddings are different
    assert_ne!(embedding1, embedding3, "Different inputs should produce different embeddings");
}

#[tokio::test]
async fn test_embedder_performance() {
    // Create a mock embedder with no latency for performance testing
    let embedder = MockEmbedder::new(
        1536, // OpenAI-like dimension
        1000, // High rate limit
        0,    // No latency
    );
    
    // Number of embeddings to generate
    let num_embeddings = 100;
    
    // Prepare inputs of varying lengths
    let mut inputs = Vec::with_capacity(num_embeddings);
    
    for i in 0..num_embeddings {
        // Create inputs of varying length
        let length = (i % 10 + 1) * 100; // 100 to 1000 characters
        let input = format!("function test{}() {{\n    {}\n}}", 
                           i, 
                           "x".repeat(length));
        inputs.push(input);
    }
    
    // Measure embedding generation time
    let start = std::time::Instant::now();
    
    for input in &inputs {
        embedder.embed(input);
    }
    
    let duration = start.elapsed();
    println!("Generated {} embeddings in {:?}", num_embeddings, duration);
    println!("Average time: {:?} per embedding", duration / num_embeddings as u32);
    
    // Verify all embeddings were generated
    let calls = embedder.get_embed_calls();
    assert_eq!(calls.len(), num_embeddings, "Should generate all embeddings");
}

#[tokio::test]
async fn test_embedder_error_handling() {
    // Create a mock embedder with very low rate limit to force errors
    let embedder = MockEmbedder::new(
        1536, // OpenAI-like dimension
        2,    // Rate limit after just 2 calls
        0,    // No latency
    );
    
    // Make several calls to trigger rate limiting
    for i in 0..5 {
        let input = format!("function test{}() {{}}", i);
        let embedding = embedder.embed(&input);
        
        // Even with errors, should still return an embedding of the correct dimension
        assert_eq!(embedding.len(), 1536, "Should return embedding of correct dimension even on error");
    }
    
    // Verify failures were recorded
    let failures = embedder.get_failures();
    assert_eq!(failures.len(), 3, "Should record 3 failures");
}

// This test requires a real Redis instance and API keys
#[tokio::test]
#[ignore] // Ignore by default to avoid API costs
async fn test_end_to_end_embedding_and_storage() {
    // Check if API keys are available
    let openai_key = std::env::var("OPENAI_API_KEY").ok();
    
    if openai_key.is_none() {
        println!("Skipping end-to-end test: No OpenAI API key available");
        return;
    }
    
    // Create OpenAI embedder
    let embedder = OpenAIEmbedder::new(&openai_key.unwrap(), "text-embedding-ada-002")
        .expect("Failed to create OpenAI embedder");
    
    // Create Redis vector store
    let store = indexer::vector_store::RedisVectorStore::new(
        "redis://127.0.0.1/",
        "test:end_to_end"
    );
    
    // Test inputs
    let inputs = vec![
        "function add(a, b) { return a + b; }",
        "function subtract(a, b) { return a - b; }",
        "function multiply(a, b) { return a * b; }",
        "function divide(a, b) { return a / b; }",
    ];
    
    // Generate embeddings and store them
    for (i, input) in inputs.iter().enumerate() {
        let entity_id = format!("math_function_{}", i);
        let embedding = embedder.embed(input);
        
        // Store the embedding
        store.upsert_embedding(
            &entity_id,
            &embedding,
            Some("math.js"),
            Some("function"),
        ).expect(&format!("Failed to store embedding {}", i));
    }
    
    // Query for similar functions
    let query = "function sum(a, b) { return a + b; }";
    let query_embedding = embedder.embed(query);
    
    let results = store.similarity_search(&query_embedding, 2);
    
    // Should find the add function as most similar
    assert!(!results.is_empty(), "Should return search results");
    assert_eq!(results[0], "math_function_0", "Should find add function as most similar");
    
    // Clean up
    let client = RedisClient::new(RedisConfig::from_url("redis://127.0.0.1/").unwrap());
    let _ = client.connect();
    let _ = client.wait_for_connect().await;
    
    // Get all keys with the test prefix
    let keys: Vec<String> = client.keys(format!("{}", store.get_key_prefix())).await
        .expect("Failed to get keys");
    
    // Delete the keys if any exist
    if !keys.is_empty() {
        let _ = client.del(keys).await
            .expect("Failed to delete keys");
    }
    
    let _ = client.quit().await;
}
