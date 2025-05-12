//! RED test for logging all Redis/vector DB operations

use indexer::vector_store::{RedisVectorStore, VectorStore};
use std::env;
use std::sync::{Arc, Mutex};
use log::{Level, Record, Metadata, SetLoggerError, LevelFilter};

struct TestLogger(Arc<Mutex<Vec<String>>>);

impl log::Log for TestLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Info
    }
    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            self.0.lock().unwrap().push(format!("{}: {}", record.level(), record.args()));
        }
    }
    fn flush(&self) {}
}

static mut LOGGER: Option<&'static TestLogger> = None;

fn init_logger(logs: Arc<Mutex<Vec<String>>>) -> Result<(), SetLoggerError> {
    let logger = Box::leak(Box::new(TestLogger(logs)));
    unsafe {
        LOGGER = Some(logger);
        log::set_logger(logger).map(|()| log::set_max_level(LevelFilter::Info))
    }
}

#[test]
fn test_redis_vector_store_logs_upsert() {
    let logs = Arc::new(Mutex::new(Vec::new()));
    init_logger(logs.clone()).unwrap();
    let redis_url = env::var("REDIS_URL").unwrap_or_else(|_| "redis://localhost:6379/0".to_string());
    let key_prefix = "code:testproject";
    let store = RedisVectorStore::new(&redis_url, key_prefix);
    
    // Use the trait method which logs operations
    let entity_id = "foo";
    let vector = vec![1.0, 2.0, 3.0];
    let _ = VectorStore::upsert_embedding(&store, entity_id, &vector, Some("test.py"), Some("doc"));
    
    let logs = logs.lock().unwrap();
    let found = logs.iter().any(|l| l.contains("VectorStore") && l.contains("upsert"));
    assert!(found, "Vector store operations should log info messages");
}
