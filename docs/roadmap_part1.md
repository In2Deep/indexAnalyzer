# STRICT ROADMAP: Rust Migration of code_indexer.py

**READ THIS ENTIRE DOCUMENT BEFORE WRITING ANY CODE. DO NOT DEVIATE FROM ANY INSTRUCTION.**

---

## 0. Critical Mandates

- The model MUST read, parse, and fully understand the logic and intent of the Python code (`code_indexer.py`) before writing any Rust code. Parroting or direct line-by-line conversion is strictly forbidden and will result in a broken port.
- You are to port `code_indexer.py` to Rust, producing a functionally identical CLI tool named `code_indexer_rust`.
- **NO creative deviation or architectural improvisation is permitted.**
- **NO new libraries or patterns may be introduced beyond those explicitly listed.**
- **ALL stdout output, especially in generate mode, MUST match the Python script byte-for-byte.**
- **ALL error handling, logging, and configuration MUST follow the instructions below exactly.**
- **ALL code must be fully async, robust, idiomatic, and tested as it is written. No stubs, TODOs, or future work sections.**

---

## 1. Project Initialization and Structure

1.1. Initialize a new Rust binary project:
```
cargo new code_indexer_rust --bin
```
1.2. Organize code into these modules in `src/`:
- `main.rs`: Entrypoint, top-level orchestration, `#[tokio::main]`.
- `cli.rs`: CLI parsing using `clap` (see section 5).
- `config.rs`: Constants, env var logic (see section 6).
- `ast_parser.rs`: Python AST parsing and entity extraction (`rustpython-ast`).
- `redis_ops.rs`: Redis logic (`fred` crate, see section 8).
- `file_processing.rs`: File system traversal (`ignore` crate).
- `error.rs`: `AppError` enum using `thiserror`.

**DO NOT** add or remove modules. **DO NOT** change file names.

---

## 2. Cargo.toml Setup (Latest Stable, May 2025)

2.1. Use these exact dependencies (latest patch version for each major as of May 2025):

```toml
[package]
name = "code_indexer_rust"
version = "0.1.0"
edition = "2021"

[dependencies]
tokio = { version = "1.37", features = ["full"] }           # Async runtime; latest 1.x
fred = { version = "10.0.0", features = ["serde_json", "tokio-runtime"] } # Redis client, latest stable
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
serde_yaml = "0.9"                                            # For YAML config parsing
ignore = "0.4"
rustpython-ast = "0.4.0"                                     # Use latest stable
log = "0.4"
fern = "0.6"
once_cell = "1.19"
dirs-next = "2.0"                                            # For home directory resolution
clap = { version = "4.5", features = ["derive"] }
thiserror = "1.0"
```

**Rationale:**
- All libraries are the latest stable and actively maintained as of May 2025.
- `serde_yaml` is used for reading YAML config files.
- `dirs-next` is used to expand `~` to the user's home directory reliably.
- `fred` 10.x is chosen for Redis due to improved performance and API stability.
- No `.env` or dotenv crates—**all configuration is via YAML file, or Rust defaults if missing**.

**DO NOT** use any crate not listed above. **DO NOT** change features.

---

## 4. Constants and Configuration (config.rs)

4.1. **DO NOT** hardcode values in multiple places. Use these constants and config strategy:
- Configuration MUST be loaded from `~/.indexer/config.yaml` (YAML format). If not found, fall back to Rust-side hardcoded defaults.
- Example config struct:

```rust
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct AppConfig {
    pub redis_url: Option<String>,
    pub log_level: Option<String>,
    // Add more keys as needed
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            redis_url: Some("redis://127.0.0.1:6379/0".to_string()),
            log_level: Some("info".to_string()),
        }
    }
}
```

- Use `dirs_next` to resolve the home directory.
- If the config file is missing, proceed with Rust defaults. If present but invalid, print a clear error and exit nonzero.
- **DO NOT** use `.env`, CLI args for config, or any other config source.
- All config keys and their defaults must be documented in code and user-facing docs.


## 3. Error Handling (error.rs)

3.1. Define a public `AppError` enum using `thiserror`:
- Must cover: I/O, Redis, AST parsing, CLI arg, Config, Logging, FileNotFound, Serialization, and YAML config errors.
- All fallible functions MUST return `Result<T, AppError>`.
- **DO NOT** use `.unwrap()` or `.expect()` outside of `main` or tests.
- `main` must print errors to stderr and exit non-zero on error.

**EXACT CODE STRUCTURE:**
```rust
use thiserror::Error;
use std::path::PathBuf;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Redis client error: {0}")]
    Redis(#[from] fred::error::RedisError),
    #[error("AST parsing error in file '{file_path}': {details}")]
    AstParse { file_path: String, details: String },
    #[error("CLI argument error: {0}")]
    CliArgument(String),
    #[error("Configuration error: {0}")]
    Config(String),
    #[error("Failed to initialize logger: {0}")]
    LoggingInitialization(String),
    #[error("File not found: {path}")]
    FileNotFound { path: PathBuf },
    #[error("JSON serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
    #[error("YAML configuration error: {0}")]
    YamlConfig(#[from] serde_yaml::Error),
}
```

---

## 4. Constants and Configuration (config.rs)

4.1. **DO NOT** hardcode values in multiple places. Use these constants:
- `KEY_PREFIX`: `"code_context:auto_agent"`
- `PROCESSED_FILES_KEY`: `"code_context:auto_agent:processed_files"`
- `SKIP_DIRS`: Use `once_cell::sync::Lazy<HashSet<&'static str>>` with these values:
  - `.logs`, `.venv`, `.git`, `__pycache__`, `node_modules`, `build`, `dist`
- `get_redis_url()`: Reads `REDIS_URL` env var, defaults to `redis://127.0.0.1:6379/0`.

**DO NOT** add, remove, or rename constants.

---

(Continued in roadmap_part2.md...)
