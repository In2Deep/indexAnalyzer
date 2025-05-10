//! logging setup for indexer

use crate::config::AppConfig;
use fern::Dispatch;
use log::LevelFilter;
use std::io;

pub fn setup_logging(config: &AppConfig) -> Result<(), io::Error> {
    let log_level = config.log_level.as_deref().unwrap_or("info");
    let level_filter = match log_level {
        "debug" => LevelFilter::Debug,
        "warn" => LevelFilter::Warn,
        "error" => LevelFilter::Error,
        "trace" => LevelFilter::Trace,
        _ => LevelFilter::Info,
    };

    Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "{} [{}] {}",
                chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
                record.level(),
                message
            ))
        })
        .level(level_filter)
        .chain(std::io::stderr())
        .apply()
        .map_err(|_| io::Error::new(io::ErrorKind::Other, "failed to initialize logger"))
}
