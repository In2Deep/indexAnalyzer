# STRICT ROADMAP (CONTINUED)

## 5. CLI Argument Parsing (cli.rs)

5.1. Use `clap` with its derive feature. Define only the following:
- `CliArgs` struct (see code below)
- `OperationMode` enum (see code below)
- Implement `Display` for `OperationMode` as shown

**EXACT CODE STRUCTURE:**
```rust
use clap::{Parser, ValueEnum};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None, formatter_class = clap::builder::RawTextHelpFormatter::new())]
pub struct CliArgs {
    #[arg(help = "Path to the application directory to index (e.g., /path/to/your/app_dir)")]
    pub app_dir: PathBuf,
    #[arg(
        value_enum,
        default_value_t = OperationMode::Generate,
        help = "Operation mode:\n  generate: Print MCP 'set'/'sadd' commands to stdout (default).\n  direct:   Connect to Redis directly and execute SET/SADD commands."
    )]
    pub mode: OperationMode,
}

#[derive(ValueEnum, Clone, Debug, PartialEq, Eq)]
pub enum OperationMode {
    Generate,
    Direct,
}

impl std::fmt::Display for OperationMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OperationMode::Generate => write!(f, "generate"),
            OperationMode::Direct => write!(f, "direct"),
        }
    }
}
```
- Validate `app_dir` exists (clap or manual check in `main`).
- Use help/description text from Python script and argparse.

---

## 6. Logging Setup (main.rs)

6.1. Use `fern` for dual logging (console + file):
- Log to stderr and to `.logs/code_indexer.log` inside `app_dir`.
- Console format: `LEVEL: message` (e.g., `INFO: Processing file...`)
- File format: `YYYY-MM-DD HH:MM:SS - LEVEL - message`
- File logging mode is 'write' (overwrite on each run).
- Log level is INFO for both.
- On logger init error, return `AppError::LoggingInitialization`.

**DO NOT** use any other logger.

---

## 7. File System Traversal (file_processing.rs)

7.1. Use `ignore::WalkBuilder`:
- Must respect `.gitignore` by default.
- **MUST** filter out `SKIP_DIRS` using a closure with `filter_entry`.
- Only process files ending in `.py`.
- Collect valid files into `Vec<PathBuf>`.

---

(Continued in roadmap_part3.md...)
