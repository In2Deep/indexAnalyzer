//! RED test for logging of all query parameters/results
use log::{Level, Record, Metadata, SetLoggerError, LevelFilter};
use std::sync::{Arc, Mutex};

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
fn test_query_logging() {
    let logs = Arc::new(Mutex::new(Vec::new()));
    init_logger(logs.clone()).unwrap();
    let query = "search for foo";
    let params = "top_k=2";
    log::info!("Query: {} Params: {}", query, params);
    let logs = logs.lock().unwrap();
    assert!(logs.iter().any(|l| l.contains("Query: search for foo")));
    assert!(logs.iter().any(|l| l.contains("Params: top_k=2")));
}
