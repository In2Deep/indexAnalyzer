//! Extreme tests for the vectorize command with pathological cases

use indexer::cli::{CliArgs, Commands};
use indexer::embedder::Embedder;
use indexer::vector_store::VectorStore;
use std::collections::HashMap;
use std::sync::{Arc, Mutex, RwLock};
use std::thread;
use std::time::{Duration, Instant};
use tempfile;

// Mock embedder that simulates network timeouts and corrupted responses
struct UnreliableEmbedder {
    // Control behavior
    timeout_frequency: f64, // 0.0 to 1.0
    corruption_frequency: f64, // 0.0 to 1.0
    retry_count: Arc<Mutex<HashMap<String, usize>>>, // Track retries per input
    max_retries: usize,
    // Track calls
    embed_calls: Arc<Mutex<Vec<String>>>,
    // Track failures
    failures: Arc<Mutex<Vec<String>>>,
    // Simulate memory pressure
    memory_usage: Arc<Mutex<usize>>,
    max_memory: usize,
}

impl UnreliableEmbedder {
    fn new(timeout_freq: f64, corruption_freq: f64, max_retries: usize, max_memory: usize) -> Self {
        UnreliableEmbedder {
            timeout_frequency: timeout_freq,
            corruption_frequency: corruption_freq,
            retry_count: Arc::new(Mutex::new(HashMap::new())),
            max_retries,
            embed_calls: Arc::new(Mutex::new(Vec::new())),
            failures: Arc::new(Mutex::new(Vec::new())),
            memory_usage: Arc::new(Mutex::new(0)),
            max_memory,
        }
    }
    
    fn get_embed_calls(&self) -> Vec<String> {
        self.embed_calls.lock().unwrap().clone()
    }
    
    fn get_failures(&self) -> Vec<String> {
        self.failures.lock().unwrap().clone()
    }
    
    fn get_retry_counts(&self) -> HashMap<String, usize> {
        self.retry_count.lock().unwrap().clone()
    }
}

impl Embedder for UnreliableEmbedder {
    fn embed(&self, input: &str) -> Vec<f32> {
        // Record the embedding call
        self.embed_calls.lock().unwrap().push(input.to_string());
        
        // Use std::panic::catch_unwind to handle panics
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            // Simulate memory allocation
            {
                let mut memory = self.memory_usage.lock().unwrap();
                *memory += input.len();
                
                // Check if we've exceeded max memory
                if *memory > self.max_memory {
                    // Record failure
                    self.failures.lock().unwrap().push(format!("Memory limit exceeded for {}", input));
                    
                    // Simulate OOM
                    panic!("Out of memory (simulated)");
                }
            }
            
            // Check if this should timeout
            // Use deterministic approach instead of random
            let should_timeout = self.embed_calls.lock().unwrap().len() % 5 == 0 && self.timeout_frequency > 0.0;
            if should_timeout {
                // Track retry count
                let retry_key = input.to_string();
                let current_retries = {
                    let mut retries = self.retry_count.lock().unwrap();
                    let count = retries.entry(retry_key.clone()).or_insert(0);
                    *count += 1;
                    *count
                };
                
                // If we've exceeded max retries, fail permanently
                if current_retries > self.max_retries {
                    // Record failure
                    self.failures.lock().unwrap().push(format!("Max retries exceeded for {}", input));
                    
                    panic!("Network timeout after max retries (simulated)");
                }
                
                // Simulate timeout
                thread::sleep(Duration::from_millis(100));
                panic!("Network timeout (simulated)");
            }
            
            // Check if this should return corrupted data
            // Use deterministic approach instead of random
            let should_corrupt = self.embed_calls.lock().unwrap().len() % 7 == 0 && self.corruption_frequency > 0.0;
            if should_corrupt {
                // Record failure
                self.failures.lock().unwrap().push(format!("Corrupted response for {}", input));
                
                // Return corrupted embedding (all zeros)
                return vec![0.0; 3];
            }
            
            // Free some memory
            {
                let mut memory = self.memory_usage.lock().unwrap();
                if *memory > input.len() {
                    *memory -= input.len();
                } else {
                    *memory = 0;
                }
            }
            
            // Return valid embedding
            vec![0.1, 0.2, 0.3]
        }));
        
        match result {
            Ok(embedding) => embedding,
            Err(_) => {
                // Record failure
                self.failures.lock().unwrap().push(format!("Panic during embedding of {}", input));
                
                // Return empty embedding on panic
                vec![0.0, 0.0, 0.0]
            }
        }
    }
}

// Mock store that simulates network partitions and race conditions
struct UnreliableVectorStore {
    // Control behavior
    partition_frequency: f64, // 0.0 to 1.0
    race_condition_frequency: f64, // 0.0 to 1.0
    // Actual storage
    stored_entities: Arc<RwLock<HashMap<String, Vec<f32>>>>,
    stored_metadata: Arc<RwLock<HashMap<String, HashMap<String, String>>>>,
    // Track operations
    store_calls: Arc<Mutex<Vec<String>>>,
    query_calls: Arc<Mutex<Vec<String>>>,
    // Track failures
    failures: Arc<Mutex<Vec<String>>>,
    // Last operation timestamp for simulating race conditions
    last_operation: Arc<Mutex<Instant>>,
}

impl UnreliableVectorStore {
    fn new(partition_freq: f64, race_freq: f64) -> Self {
        UnreliableVectorStore {
            partition_frequency: partition_freq,
            race_condition_frequency: race_freq,
            stored_entities: Arc::new(RwLock::new(HashMap::new())),
            stored_metadata: Arc::new(RwLock::new(HashMap::new())),
            store_calls: Arc::new(Mutex::new(Vec::new())),
            query_calls: Arc::new(Mutex::new(Vec::new())),
            failures: Arc::new(Mutex::new(Vec::new())),
            last_operation: Arc::new(Mutex::new(Instant::now())),
        }
    }
    
    #[allow(dead_code)]
    fn get_stored_count(&self) -> usize {
        match self.stored_entities.read() {
            Ok(entities) => entities.len(),
            Err(_) => 0, // RwLock poisoned
        }
    }
    
    fn get_store_calls(&self) -> Vec<String> {
        self.store_calls.lock().unwrap().clone()
    }
    
    #[allow(dead_code)]
    fn get_query_calls(&self) -> Vec<String> {
        self.query_calls.lock().unwrap().clone()
    }
    
    fn get_failures(&self) -> Vec<String> {
        self.failures.lock().unwrap().clone()
    }
}

impl VectorStore for UnreliableVectorStore {
    fn upsert_embedding(&self, entity_id: &str, embedding: &[f32], file: Option<&str>, entity_type: Option<&str>) -> Result<(), String> {
        // Record the store call
        self.store_calls.lock().unwrap().push(entity_id.to_string());
        
        // Check for network partition
        // Use deterministic approach instead of random
let should_partition = self.store_calls.lock().unwrap().len() % 6 == 0 && self.partition_frequency > 0.0;
if should_partition {
            // Record failure
            self.failures.lock().unwrap().push(format!("Network partition during store of {}", entity_id));
            
            return Err("Network partition (simulated)".to_string());
        }
        
        // Check for race condition
        {
            let mut last_op = self.last_operation.lock().unwrap();
            let now = Instant::now();
            let elapsed = now.duration_since(*last_op);
            
            if elapsed < Duration::from_millis(10) {
                // Use deterministic approach instead of random
                let should_race = self.store_calls.lock().unwrap().len() % 8 == 0 && self.race_condition_frequency > 0.0;
                if should_race {
                    // Record failure
                    self.failures.lock().unwrap().push(format!("Race condition during store of {}", entity_id));
                    
                    return Err("Race condition detected (simulated)".to_string());
                }
            }
            
            *last_op = now;
        }
        
        // Store the embedding
        match self.stored_entities.write() {
            Ok(mut entities) => {
                entities.insert(entity_id.to_string(), embedding.to_vec());
            },
            Err(_) => {
                // RwLock poisoned
                self.failures.lock().unwrap().push(format!("RwLock poisoned during store of {}", entity_id));
                return Err("RwLock poisoned (simulated)".to_string());
            }
        }
        
        // Store metadata
        match self.stored_metadata.write() {
            Ok(mut metadata_map) => {
                let mut metadata = HashMap::new();
                metadata.insert("id".to_string(), entity_id.to_string());
                if let Some(f) = file {
                    metadata.insert("file".to_string(), f.to_string());
                }
                if let Some(et) = entity_type {
                    metadata.insert("type".to_string(), et.to_string());
                }
                metadata_map.insert(entity_id.to_string(), metadata);
            },
            Err(_) => {
                // RwLock poisoned
                self.failures.lock().unwrap().push(format!("RwLock poisoned during metadata store of {}", entity_id));
                return Err("RwLock poisoned (simulated)".to_string());
            }
        }
        
        Ok(())
    }
    
    fn similarity_search(&self, _query: &[f32], _top_k: usize) -> Vec<String> {
        // Record the query call
        self.query_calls.lock().unwrap().push("similarity_search".to_string());
        
        // Check for network partition
        // Use deterministic approach instead of random
let should_partition = self.store_calls.lock().unwrap().len() % 6 == 0 && self.partition_frequency > 0.0;
if should_partition {
            // Record failure
            self.failures.lock().unwrap().push("Network partition during similarity search".to_string());
            
            return Vec::new();
        }
        
        // Return all stored entity IDs
        match self.stored_entities.read() {
            Ok(entities) => entities.keys().cloned().collect(),
            Err(_) => {
                // RwLock poisoned
                self.failures.lock().unwrap().push("RwLock poisoned during similarity search".to_string());
                Vec::new()
            }
        }
    }
    
    fn get_all_entity_ids(&self) -> Result<Vec<String>, String> {
        // Record the query call
        self.query_calls.lock().unwrap().push("get_all_entity_ids".to_string());
        
        // Check for network partition
        // Use deterministic approach instead of random
let should_partition = self.store_calls.lock().unwrap().len() % 6 == 0 && self.partition_frequency > 0.0;
if should_partition {
            // Record failure
            self.failures.lock().unwrap().push("Network partition during get_all_entity_ids".to_string());
            
            return Err("Network partition (simulated)".to_string());
        }
        
        match self.stored_entities.read() {
            Ok(entities) => Ok(entities.keys().cloned().collect()),
            Err(_) => {
                // RwLock poisoned
                self.failures.lock().unwrap().push("RwLock poisoned during get_all_entity_ids".to_string());
                Err("RwLock poisoned (simulated)".to_string())
            }
        }
    }
    
    fn get_entity_vector(&self, entity_id: &str) -> Result<Vec<f32>, String> {
        // Record the query call
        self.query_calls.lock().unwrap().push(format!("get_entity_vector:{}", entity_id));
        
        // Check for network partition
        // Use deterministic approach instead of random
let should_partition = self.store_calls.lock().unwrap().len() % 6 == 0 && self.partition_frequency > 0.0;
if should_partition {
            // Record failure
            self.failures.lock().unwrap().push(format!("Network partition during get_entity_vector of {}", entity_id));
            
            return Err("Network partition (simulated)".to_string());
        }
        
        match self.stored_entities.read() {
            Ok(entities) => entities.get(entity_id)
                .cloned()
                .ok_or_else(|| format!("Entity not found: {}", entity_id)),
            Err(_) => {
                // RwLock poisoned
                self.failures.lock().unwrap().push(format!("RwLock poisoned during get_entity_vector of {}", entity_id));
                Err("RwLock poisoned (simulated)".to_string())
            }
        }
    }
    
    fn get_entity_metadata(&self, entity_id: &str) -> Result<HashMap<String, String>, String> {
        // Record the query call
        self.query_calls.lock().unwrap().push(format!("get_entity_metadata:{}", entity_id));
        
        // Check for network partition
        // Use deterministic approach instead of random
let should_partition = self.store_calls.lock().unwrap().len() % 6 == 0 && self.partition_frequency > 0.0;
if should_partition {
            // Record failure
            self.failures.lock().unwrap().push(format!("Network partition during get_entity_metadata of {}", entity_id));
            
            return Err("Network partition (simulated)".to_string());
        }
        
        match self.stored_metadata.read() {
            Ok(metadata) => metadata.get(entity_id)
                .cloned()
                .ok_or_else(|| format!("Entity metadata not found: {}", entity_id)),
            Err(_) => {
                // RwLock poisoned
                self.failures.lock().unwrap().push(format!("RwLock poisoned during get_entity_metadata of {}", entity_id));
                Err("RwLock poisoned (simulated)".to_string())
            }
        }
    }
}

// Create a file with deeply nested structures
fn create_deeply_nested_file(path: &std::path::Path) -> std::io::Result<()> {
    let mut content = String::new();
    
    // Add file header
    content.push_str("//! A file with deeply nested structures\n\n");
    
    // Add a deeply nested structure (10 levels deep)
    content.push_str("mod outer {\n");
    for i in 0..10 {
        content.push_str(&"    ".repeat(i + 1));
        content.push_str(&format!("mod level_{} {{\n", i));
    }
    
    // Add some functions at the deepest level
    for i in 0..5 {
        content.push_str(&"    ".repeat(11));
        content.push_str(&format!("fn deep_function_{0}() {{\n", i));
        content.push_str(&"    ".repeat(12));
        content.push_str(&format!("println!(\"Deep function {0}\");\n", i));
        content.push_str(&"    ".repeat(11));
        content.push_str("}\n\n");
    }
    
    // Close all the nested modules
    for i in (0..10).rev() {
        content.push_str(&"    ".repeat(i + 1));
        content.push_str("}\n");
    }
    content.push_str("}\n");
    
    std::fs::write(path, content)
}

// Create a file with circular references
fn create_circular_reference_files(dir: &std::path::Path) -> std::io::Result<()> {
    // File A references File B
    let file_a_path = dir.join("file_a.rs");
    let file_a_content = r#"
//! File A that references File B

mod file_b;

pub fn function_a() {
    println!("Function A calling Function B");
    file_b::function_b();
}
"#;
    std::fs::write(&file_a_path, file_a_content)?;
    
    // File B references File A
    let file_b_path = dir.join("file_b.rs");
    let file_b_content = r#"
//! File B that references File A

mod file_a;

pub fn function_b() {
    println!("Function B calling Function A");
    file_a::function_a();
}
"#;
    std::fs::write(&file_b_path, file_b_content)?;
    
    Ok(())
}

#[tokio::test]
async fn test_vectorize_command_with_unreliable_components() {
    // Create a temporary directory
    let temp_dir = tempfile::tempdir().unwrap();
    
    // Create a few Rust files
    for i in 0..5 {
        let file_path = temp_dir.path().join(format!("file{}.rs", i));
        let file_content = format!(r#"
//! File {0} with some functions

fn function_{0}_1() {{
    println!("Function {0}_1");
}}

fn function_{0}_2() {{
    println!("Function {0}_2");
}}
"#, i);
        std::fs::write(&file_path, file_content).unwrap();
    }
    
    // Create a deeply nested file
    let nested_file_path = temp_dir.path().join("nested.rs");
    create_deeply_nested_file(&nested_file_path).unwrap();
    
    // Create files with circular references
    create_circular_reference_files(temp_dir.path()).unwrap();
    
    // Setup unreliable components with moderate failure rates
    let embedder = UnreliableEmbedder::new(
        0.2,  // 20% timeout frequency
        0.1,  // 10% corruption frequency
        3,    // Max 3 retries
        1024, // Max 1KB memory
    );
    
    let store = UnreliableVectorStore::new(
        0.15, // 15% partition frequency
        0.1   // 10% race condition frequency
    );
    
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
    let result = indexer::vectorize_command(&args, &embedder, &store).await;
    
    // Our implementation should handle unreliable components gracefully
    // It's acceptable for it to either:
    // 1. Complete successfully despite failures (with proper error handling)
    // 2. Fail with a meaningful error message
    
    match result {
        Ok(_) => {
            println!("Command succeeded despite unreliable components");
            
            // Check what was processed
            let embed_calls = embedder.get_embed_calls();
            let embed_failures = embedder.get_failures();
            let store_calls = store.get_store_calls();
            let store_failures = store.get_failures();
            
            println!("Processed {} embeddings", embed_calls.len());
            println!("Embedding failures: {}", embed_failures.len());
            println!("Store calls: {}", store_calls.len());
            println!("Store failures: {}", store_failures.len());
            
            // Check retry behavior
            let retries = embedder.get_retry_counts();
            if !retries.is_empty() {
                println!("Retry counts: {:?}", retries);
            }
        },
        Err(e) => {
            println!("Command failed with error: {}", e);
            
            // Error should be meaningful
            assert!(!e.is_empty(), "Error message should not be empty");
        }
    }
    
    // Regardless of success or failure, we should have attempted to process some files
    let embed_calls = embedder.get_embed_calls();
    assert!(!embed_calls.is_empty(), "Should have attempted to process at least some files");
}

#[tokio::test]
async fn test_vectorize_command_with_extreme_batch_sizes() {
    // Create a temporary directory
    let temp_dir = tempfile::tempdir().unwrap();
    
    // Create 100 small Rust files
    for i in 0..100 {
        let file_path = temp_dir.path().join(format!("small_file{}.rs", i));
        let file_content = format!(r#"
fn small_function_{0}() {{
    println!("Small function {0}");
}}
"#, i);
        std::fs::write(&file_path, file_content).unwrap();
    }
    
    // Setup mock components
    let embedder = UnreliableEmbedder::new(
        0.0,     // No timeouts
        0.0,     // No corruptions
        0,       // No retries needed
        1024*1024 // Plenty of memory
    );
    
    let store = UnreliableVectorStore::new(
        0.0, // No partitions
        0.0  // No race conditions
    );
    
    // Test with tiny batch size
    let tiny_batch_args = CliArgs {
        command: Commands::Vectorize {
            name: "tiny_batch".to_string(),
            path: temp_dir.path().to_string_lossy().to_string(),
            provider: Some("mock".to_string()),
            db: Some("redis".to_string()),
            batch_size: Some(1), // Tiny batch size
            dry_run: false,
            verbose: true,
        },
    };
    
    // Call the vectorize command function with tiny batch size
    let tiny_result = indexer::vectorize_command(&tiny_batch_args, &embedder, &store).await;
    
    // Test with huge batch size
    let huge_batch_args = CliArgs {
        command: Commands::Vectorize {
            name: "huge_batch".to_string(),
            path: temp_dir.path().to_string_lossy().to_string(),
            provider: Some("mock".to_string()),
            db: Some("redis".to_string()),
            batch_size: Some(1000), // Huge batch size
            dry_run: false,
            verbose: true,
        },
    };
    
    // Call the vectorize command function with huge batch size
    let huge_result = indexer::vectorize_command(&huge_batch_args, &embedder, &store).await;
    
    // Both should complete successfully
    assert!(tiny_result.is_ok(), "Command should succeed with tiny batch size");
    assert!(huge_result.is_ok(), "Command should succeed with huge batch size");
    
    // Check what was processed
    let embed_calls = embedder.get_embed_calls();
    let store_calls = store.get_store_calls();
    
    println!("Processed {} embeddings", embed_calls.len());
    println!("Store calls: {}", store_calls.len());
    
    // We should have processed a significant number of files
    assert!(embed_calls.len() > 10, "Should have processed multiple files");
}
