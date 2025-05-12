//! RED tests for batch processing of vector embeddings with progress logging
//! Tests batch processing, error handling, and progress logging

use indexer::embedder::MockEmbedder;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use log::Level;

// Function we're testing - will need to be implemented
// This is just a signature to make the tests compile
extern crate indexer;

#[test]
fn test_batch_process_with_progress() {
    // Create a test logger to capture log messages
    let (logger, logs) = test_logger::TestLogger::new();
    log::set_boxed_logger(Box::new(logger)).unwrap();
    log::set_max_level(log::LevelFilter::Info);
    
    let texts = vec![
        "def func1(): pass",
        "def func2(): pass",
        "class Class1: pass",
    ];
    
    let embedder = MockEmbedder;
    let progress_counter = Arc::new(AtomicUsize::new(0));
    let progress_clone = Arc::clone(&progress_counter);
    
    // Define a progress callback
    let progress_callback = move |current: usize, total: usize| {
        progress_clone.store(current, Ordering::SeqCst);
        log::info!("Processed {} of {} entities", current, total);
    };
    
    // Call the batch_process_entities function that needs to be implemented
    let result = indexer::batch_process_entities(
        &texts,
        &embedder,
        progress_callback
    );
    
    // Verify the result
    assert!(result.is_ok(), "Batch processing should succeed");
    
    // Verify progress was tracked correctly
    let final_progress = progress_counter.load(Ordering::SeqCst);
    assert!(final_progress > 0, "Progress counter should be updated");
    
    // Verify logs contain progress information
    let logs = logs.lock().unwrap();
    let progress_logs = logs.iter()
        .filter(|(level, msg)| *level == Level::Info && msg.contains("Processed"))
        .collect::<Vec<_>>();
    
    assert!(!progress_logs.is_empty(), "Should have progress log messages");
    assert!(progress_logs.iter().any(|(_, msg)| msg.contains("Processed")), 
            "Log messages should include progress information");
}

#[test]
fn test_batch_process_error_handling() {
    let texts = vec![
        "def good_func(): pass",
        "", // Empty text to trigger an error
        "class GoodClass: pass",
    ];
    
    let embedder = MockEmbedder;
    
    // Define a no-op progress callback
    let progress_callback = |_: usize, _: usize| {};
    
    // Call the batch_process_entities function
    let result = indexer::batch_process_entities(
        &texts,
        &embedder,
        progress_callback
    );
    
    // Verify the result contains errors but doesn't fail completely
    assert!(result.is_ok(), "Batch processing should handle errors gracefully");
    
    let (embeddings, errors) = result.unwrap();
    
    // We should have embeddings for the good texts
    assert!(embeddings.len() >= 2, "Should have embeddings for good texts");
    
    // We should have an error for the empty text
    assert!(!errors.is_empty(), "Should have errors for problematic texts");
}

// Helper module for testing logging
mod test_logger {
    use log::{Record, Level, Metadata};
    use std::sync::{Arc, Mutex};
    
    pub struct TestLogger {
        logs: Arc<Mutex<Vec<(Level, String)>>>,
    }
    
    impl TestLogger {
        pub fn new() -> (Self, Arc<Mutex<Vec<(Level, String)>>>) {
            let logs = Arc::new(Mutex::new(Vec::new()));
            (TestLogger { logs: Arc::clone(&logs) }, logs)
        }
    }
    
    impl log::Log for TestLogger {
        fn enabled(&self, _metadata: &Metadata) -> bool {
            true
        }
        
        fn log(&self, record: &Record) {
            let mut logs = self.logs.lock().unwrap();
            logs.push((record.level(), record.args().to_string()));
        }
        
        fn flush(&self) {}
    }
}
