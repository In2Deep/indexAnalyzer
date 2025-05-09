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
- Initialize a new Rust binary project:
  ```
  cargo new code_indexer_rust --bin
  ```
- Organize code into these modules in `src/`:
  - `main.rs`: Entrypoint, top-level orchestration, `#[tokio::main]`.
  - `cli.rs`: CLI parsing using `clap` (see section 5).
  - `config.rs`: Constants, env var logic (see section 6).
  - `ast_parser.rs`: Python AST parsing and entity extraction (`rustpython-ast`).
  - `redis_ops.rs`: Redis logic (`fred` crate, see section 8).
  - `file_processing.rs`: File system traversal (`ignore` crate).
  - `error.rs`: `AppError` enum using `thiserror`.
- **DO NOT** add or remove modules. **DO NOT** change file names.

---

## 2. Cargo.toml Setup (Latest Stable, May 2025)
- Use these exact dependencies (latest patch version for each major as of May 2025):
  ```toml
  [package]
  name = "code_indexer_rust"
  version = "0.1.0"
  edition = "2021"

  [dependencies]
  tokio = { version = "1.37", features = ["full"] }
  fred = { version = "10.1.0", features = ["serde_json", "tokio-runtime"] }
  serde = { version = "1.0", features = ["derive"] }
  serde_json = "1.0"
  serde_yaml = "0.9"
  ignore = "0.4"
  rustpython-ast = "0.4.0"
  log = "0.4"
  fern = "0.6"
  once_cell = "1.19"
  dirs-next = "2.0"
  clap = { version = "4.5", features = ["derive"] }
  thiserror = "1.0"
  ```
- **Rationale:**
  - All libraries are the latest stable and actively maintained as of May 2025.
  - `serde_yaml` is used for reading YAML config files.
  - `dirs-next` is used to expand `~` to the user's home directory reliably.
  - `fred` 10.1.0 is chosen for Redis due to improved performance and API stability.
  - No `.env` or dotenv crates—**all configuration is via YAML file, or Rust defaults if missing**.
- **DO NOT** use any crate not listed above. **DO NOT** change features.

---

## 3. Error Handling (error.rs)
- Define a public `AppError` enum using `thiserror`:
  - Must cover: I/O, Redis, AST parsing, CLI arg, Config, Logging, FileNotFound, Serialization, and YAML config errors.
  - All fallible functions MUST return `Result<T, AppError>`.
  - **DO NOT** use `.unwrap()` or `.expect()` outside of `main` or tests.
  - `main` must print errors to stderr and exit non-zero on error.
- **EXACT CODE STRUCTURE:**
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
- **DO NOT** hardcode values in multiple places. Use these constants and config strategy:
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

---

## 5. CLI Argument Parsing (cli.rs)
- Use `clap` with its derive feature. Define only the following:
  - `CliArgs` struct (see code below)
  - `OperationMode` enum (see code below)
  - Implement `Display` for `OperationMode` as shown
- **EXACT CODE STRUCTURE:**
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
- Use `fern` for dual logging (console + file):
  - Log to stderr and to `.logs/code_indexer.log` inside `app_dir`.
  - Console format: `LEVEL: message` (e.g., `INFO: Processing file...`)
  - File format: `YYYY-MM-DD HH:MM:SS - LEVEL - message`
  - File logging mode is 'write' (overwrite on each run).
  - Log level is INFO for both.
  - On logger init error, return `AppError::LoggingInitialization`.
- **DO NOT** use any other logger.

---

## 7. File System Traversal (file_processing.rs)
- Use `ignore::WalkBuilder`:
  - Must respect `.gitignore` by default.
  - **MUST** filter out `SKIP_DIRS` using a closure with `filter_entry`.
  - Only process files ending in `.py`.
  - Collect valid files into `Vec<PathBuf>`.

---

## 8. AST Parsing & Code Entity Extraction (ast_parser.rs)
- Use `rustpython-ast` for parsing. **DO NOT** use any other parser.
- Define `CodeEntity` struct exactly as below (fields/serde attributes must match):
  ```rust
  use serde::Serialize;
  #[derive(Serialize, Debug, Clone)]
  pub struct CodeEntity {
      pub entity_type: String, // "function", "class", "method", "variable"
      pub file_path: String,   // Relative path from app_dir (POSIX)
      pub name: String,
      #[serde(skip_serializing_if = "Option::is_none")]
      pub signature: Option<String>,
      #[serde(skip_serializing_if = "Option::is_none")]
      pub docstring: Option<String>,
      pub line_start: usize,
      pub line_end: usize,
      #[serde(skip_serializing_if = "Option::is_none")]
      pub parent_class: Option<String>,
      #[serde(skip_serializing_if = "Option::is_none")]
      pub bases: Option<Vec<String>>,
      #[serde(skip_serializing_if = "Option::is_none")]
      pub value_repr: Option<String>,
  }
  ```
- Implement `extract_entities_from_file(file_path: &PathBuf, app_dir: &PathBuf) -> Result<Vec<CodeEntity>, AppError>`:
  - Read file async with `tokio::fs::read_to_string`.
  - Parse with `rustpython_parser::parser::parse_program`.
  - On error, log and return `AppError::AstParse`.
  - Traverse AST recursively, passing parent context as needed (do NOT assign parent attributes).
  - Extract all required fields for each entity, as described in `promopt.md`.
  - Signature construction: **DO NOT** use line-matching heuristics. Instead, build from AST structure as described.
  - Continue on recoverable node errors; skip file on fatal parse error.

---

## 9. Redis Operations (redis_ops.rs)
- Use `fred` crate for all Redis communication.
  - Only in `direct` mode: connect, wait, ping.
  - Keys and commands must mirror Python logic:
    - If `entity_type == "method"`, key is `{key_prefix}:method:{file_path}:{parent_class}.{name}`.
    - Else, `{key_prefix}:{entity_type}:{file_path}:{name}`.
  - Implement `process_set_command` as described in `promopt.md`.
  - **DO NOT** use blocking or sync Redis calls.

---

## 10. Output & Testing
- In generate mode, output MCP commands to stdout. **MUST** match Python output byte-for-byte.
- Implement comprehensive async tests for all modules. **DO NOT** skip error paths.

---

## 11. Forbidden Actions
- **DO NOT** introduce any new libraries, patterns, or architectural changes.
- **DO NOT** add extra features, refactorings, or "improvements" unless explicitly instructed.
- **DO NOT** skip or alter any error handling, logging, or output requirements.

---

**END OF STRICT ROADMAP**

If the content above does not fit in a single file, continue splitting as needed and reference previous parts.
