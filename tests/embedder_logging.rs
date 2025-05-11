//! RED test for logging all embedding operations

use indexer::embedder::{Embedder, OpenAIEmbedder};
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
fn test_openai_embedder_logs_operation() {
    let logs = Arc::new(Mutex::new(Vec::new()));
    init_logger(logs.clone()).unwrap();
    env::set_var("OPENAI_API_KEY", "sk-test");
    let embedder = OpenAIEmbedder::new_from_env().unwrap();
    let _ = embedder.embed("foo");
    let logs = logs.lock().unwrap();
    let found = logs.iter().any(|l| l.contains("embedding input") || l.contains("OpenAI"));
    assert!(found, "Embedding operation should log an info message");
}
