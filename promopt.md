dObjective:

You are an expert Rust developer AI agent. Your task is to perform a highly precise and idiomatic port of the provided Python script (code_indexer.py) to Rust. You MUST adhere strictly to the instructions below, which are derived from an exhaustive "Analysis of Python 'code_indexer' Script and Grok's Rust Porting Assessment for Agent-Driven Migration" (referred to as "the Analysis Document" or "Analysis Doc" hereafter). No creative deviations or independent architectural decisions are permitted. The goal is a functionally identical Rust application that is robust, maintainable, and performant, meticulously following the guidance of the Analysis Document.

Python Script to be Ported:

[insert Python script here]

I. Overall Goal & Functionality (Ref: Analysis Doc Section 5.2 Overall Goal):

The resultant Rust application, named code_indexer_rust, must be a command-line tool replicating the full functionality of the Python script. It will operate in two modes:

generate mode (default): Print 'MCP' (Meta-Command Protocol) commands to standard output. The format of this output (including JSON structure, command names, arguments, comments, and "---" separators) must exactly match the Python script's generate mode output byte-for-byte (UTF-8 encoded) for identical inputs. This is critical for interoperability.
direct mode: Interact directly with a Redis server to execute SET and SADD commands.
The Rust application must:
* Accept a target application directory path (app_dir) as a command-line argument.
* Asynchronously traverse app_dir, processing only files ending with the .py extension.
* Strictly observe and filter out directories listed in the SKIP_DIRS constant (from the Python script: [see Python script for SKIP_DIRS definition]).
* Parse Python source code using the rustpython-ast crate to extract structural information (functions, classes, methods, module/class-level variables).
* Manage all I/O operations (file system, Redis) asynchronously using the tokio runtime.
* Implement logging to both standard error (console) and a file named code_indexer.log located within a .logs subdirectory of the provided app_dir. The logging behavior and format should mirror the Python script's setup_logging function [see Python script setup_logging function].
* Prioritize correctness, robustness according to Rust best practices, idiomatic Rust patterns (as detailed in the Analysis Document), and maintainability.

II. Project Structure (Ref: Analysis Doc Section 5.2 Project Structure):

Initialize a new Rust binary project: cargo new code_indexer_rust --bin.
Organize Rust code into the following logical modules within the src/ directory. Adhere to this structure:
main.rs: Main application entry point, high-level orchestration, and #[tokio::main] annotation.
cli.rs: Command-line argument parsing logic and related struct/enum definitions using clap.
config.rs: Application configuration constants (e.g., KEY_PREFIX, PROCESSED_FILES_KEY, SKIP_DIRS) and functions (e.g., get_redis_url).
ast_parser.rs: Python AST parsing, code entity definition (CodeEntity struct), and extraction logic using rustpython-ast.
redis_ops.rs: Redis interaction logic for both generate and direct modes, using the fred crate.
file_processing.rs: File system traversal using the ignore crate and orchestration of parsing individual files.
error.rs: Definition of the custom AppError enum using thiserror.
III. Cargo.toml Setup (Ref: Analysis Doc Section 5.2 Cargo.toml Setup):

Generate a Cargo.toml file with the following dependencies. Use the specified versions. If a specific minor/patch version is unknown but a major version is provided (e.g., "7.0"), use the latest stable release within that major version (e.g., "7.x.y"). Verify latest stable versions on crates.io if significant time has passed since the Analysis Document's creation (Dec 2024), but prioritize the major versions specified.

```toml
[package]
name = "code_indexer_rust"
version = "0.1.0"
edition = "2021"

[dependencies]
tokio = { version = "1.37", features = ["full"] } # Analysis: "1.35", verify latest 1.x
fred = { version = "7.2.0", features = ["serde_json", "tokio-runtime"] } # Analysis: "7.0", verify latest 7.x
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
ignore = "0.4" # Analysis: "0.4"
rustpython-ast = "0.3.1" # Analysis: "0.3.0", verify latest 0.3.x or 0.4.x if API stable. Analysis indicates rustpython-parser is brought in by this.
# If rustpython-ast does not re-export parser::parse_program, explicitly add:
# rustpython-parser-core = "0.3.1" # (match rustpython-ast version)
# rustpython-parser = "0.3.1"      # (match rustpython-ast version)
log = "0.4"
env_logger = "0.11" # Analysis: "0.10", verify latest 0.x
clap = { version = "4.5", features = ["derive"] } # Analysis: "4.4", verify latest 4.x
thiserror = "1.0"
# Add fern for dual console/file logging if env_logger is insufficient.
# Example: fern = "0.6"
# Add once_cell for static lazy initialized HashSet if preferred for SKIP_DIRS_SET.
# Example: once_cell = "1.19"
```


V. Error Handling Strategy (Ref: Analysis Doc Section 4.1 & 5.2 Error Handling):

In src/error.rs, define a comprehensive public error enum named AppError using the thiserror crate. This enum must encapsulate all potential failure modes of the application.


```
// Example structure for src/error.rs
use thiserror::Error;
use std::path::PathBuf;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Redis client error: {0}")]
    Redis(#[from] fred::error::RedisError), // Ensure this matches fred's error type

    #[error("AST parsing error in file '{file_path}': {details}")]
    AstParse { file_path: String, details: String },

    #[error("CLI argument error: {0}")]
    CliArgument(String), // Typically from clap validation or custom checks

    #[error("Configuration error: {0}")]
    Config(String),

    #[error("Failed to initialize logger: {0}")]
    LoggingInitialization(String),

    #[error("File not found: {path}")]
    FileNotFound { path: PathBuf },

    // Add other specific variants as needed, e.g., for serialization.
    #[error("JSON serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
}
```
All functions that can fail must return Result<T, AppError>.
Use the ? operator for concise error propagation.
Strictly avoid using .unwrap() or .expect() in any application logic outside of main or tests. Inside main, .expect() may only be used for critical setup failures where recovery is impossible and immediate termination is desired (e.g., failure to parse essential CLI arguments if not handled by clap's error reporting, or initial Redis connection in direct mode if the connection itself is a prerequisite for any further action).
The main function in src/main.rs must return Result<(), AppError>. If an Err is returned from the primary logic, main should print the error to stderr (e.g., eprintln!("Error: {}", e);) and terminate the process with a non-zero exit code (e.g., std::process::exit(1);).
V. Constants and Configuration (Ref: Analysis Doc Section 5.2 Constants and Config, Python script for values):

Implement in src/config.rs:

KEY_PREFIX: pub const KEY_PREFIX: &str = "code_context:auto_agent"; (Value from Python script [see Python script for KEY_PREFIX]).
PROCESSED_FILES_KEY: pub const PROCESSED_FILES_KEY: &str = "code_context:auto_agent:processed_files"; (Value derived from KEY_PREFIX in Python [see Python script for PROCESSED_FILES_KEY]).
SKIP_DIRS:
```
use std::collections::HashSet;
use once_cell::sync::Lazy; // Add once_cell to Cargo.toml if using this

pub static SKIP_DIRS: Lazy<HashSet<&'static str>> = Lazy::new(|| {
    let dirs: HashSet<&'static str> = [
        ".logs", ".venv", ".git", "__pycache__",
        "node_modules", "build", "dist"
        // Ensure this list exactly matches Python's SKIP_DIRS
        // [see Python script for SKIP_DIRS set definition]
    ].iter().cloned().collect();
    dirs
});
```

REDIS_URL: Create a function pub fn get_redis_url() -> String. This function will read the REDIS_URL environment variable. If the variable is not set, it must default to "redis://127.0.0.1:6379/0". (Logic from Python script [see Python script for REDIS_URL logic]).
VI. Logging Setup (Ref: Analysis Doc Section 2.3, 3.5, 5.2 Logging Setup, Python script setup_logging):

In src/main.rs, early in the main execution logic, initialize logging. The setup must replicate the Python script's behavior:

Log messages to standard error (console). Use env_logger for this, configured to honor the RUST_LOG environment variable.
Simultaneously, log messages to a file.
The log file must be named code_indexer.log.
This file must be located in a directory named .logs directly within the app_dir specified by the CLI argument. Create the .logs directory if it does not exist.
Consider using the fern crate for this dual console/file logging setup, as env_logger primarily targets one output. Configure fern to use distinct formatters:
Console: %(levelname)s: %(message)s (e.g., INFO: Processing file...)
File: %(asctime)s - %(levelname)s - %(message)s (e.g., 2024-12-17 10:00:00 - INFO - Processing file...)
Ensure file logging mode is 'write' (overwrite existing log file on new run, matching Python's mode='w').
Log level for both handlers should be INFO.
Use log::info!, log::warn!, log::error!, log::debug! macros throughout the application.
Handle potential errors during logger initialization by returning AppError::LoggingInitialization.
VII. CLI Argument Parsing (Ref: Analysis Doc Section 2.11, 3.5, 5.2 CLI Parsing):

Implement in src/cli.rs:

Use the clap crate with its derive feature (#[derive(Parser)]).
Define a public struct CliArgs.

```
use clap::{Parser, ValueEnum};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None, formatter_class = clap::builder::RawTextHelpFormatter::new())] // Replicate Python argparse description and formatter
pub struct CliArgs {
    #[arg(help = "Path to the application directory to index (e.g., /path/to/your/app_dir)")]
    pub app_dir: PathBuf, // Replaces TARGET_DIR from Python script

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

// Implement Display for OperationMode to satisfy default_value_t help text
impl std::fmt::Display for OperationMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OperationMode::Generate => write!(f, "generate"),
            OperationMode::Direct => write!(f, "direct"),
        }
    }
}
```

Ensure app_dir is validated by clap as an existing directory. If not, clap should handle the error. If clap doesn't do this by default for PathBuf, add a manual check early in main.rs and return AppError::CliArgument or AppError::FileNotFound. The Python script exits if TARGET_DIR is not found.
The main description for clap should be adapted from the Python script's initial docstring and argparse.ArgumentParser description. ([see Python script docstring and argparse setup]).
VIII. File System Traversal (Ref: Analysis Doc Section 2.4, 2.5, 3.3, 5.2 File System Traversal):

Implement in src/file_processing.rs:

Use the ignore crate's WalkBuilder for traversing the app_dir.
Configure WalkBuilder as follows:
It must automatically respect .gitignore files (default behavior of ignore::WalkBuilder::new(&app_dir)).
It must process hidden files/directories unless explicitly excluded by .gitignore or SKIP_DIRS. (This aligns with standard ignore crate behavior and Python's os.walk not explicitly skipping hidden files beyond SKIP_DIRS).
Crucially, implement filtering for SKIP_DIRS: The Python script prunes dirs[:] within os.walk before further processing. Replicate this priority. Add a custom filter predicate using WalkBuilder::filter_entry. This closure must:
Check if a DirEntry is a directory.
If it's a directory, check if its name is present in the config::SKIP_DIRS HashSet.
If the directory name is in SKIP_DIRS, the filter must return ignore::FilterResult::Ignore to skip this directory and its descendants.
Otherwise (not in SKIP_DIRS or not a directory), return ignore::FilterResult::뭍tch to allow the ignore crate's standard gitignore processing to continue.
Process only files:
Ensure the DirEntry is a file.
Ensure the file name ends with the .py extension.
The traversal should yield PathBuf objects for valid Python files to be processed. Collect these into a Vec<PathBuf>.
IX. AST Parsing & Code Info Extraction (Ref: Analysis Doc Section 2.6, 2.7, 3.1, 5.2 AST Parsing):

Implement in src/ast_parser.rs. This is the most complex part and requires meticulous attention to detail.

Define a public Rust struct CodeEntity (and any necessary supporting enums/structs) to represent the extracted code information. This struct must be serializable to JSON using serde. Its fields should map directly to the keys in the dictionaries appended to the entities list in the Python extract_code_info function.

```
// Example in src/ast_parser.rs
use serde::Serialize;
use std::path::PathBuf; // Or String for relative path

#[derive(Serialize, Debug, Clone)]
pub struct CodeEntity {
    pub entity_type: String, // "function", "class", "method", "variable"
    pub file_path: String,   // Relative path from app_dir (as_posix representation)
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signature: Option<String>, // For functions/methods
    #[serde(skip_serializing_if = "Option::is_none")]
    pub docstring: Option<String>, // For functions/methods/classes
    pub line_start: usize,
    pub line_end: usize,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_class: Option<String>, // For methods and class-level variables
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bases: Option<Vec<String>>, // For classes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value_repr: Option<String>, // For variables
}
```
Create a public async function, e.g., pub async fn extract_entities_from_file(file_path: &PathBuf, app_dir: &PathBuf) -> Result<Vec<CodeEntity>, AppError>, that:
Takes the absolute file_path of a Python script and the app_dir PathBuf.
Calculates the relative path string (from app_dir to file_path, POSIX format) for CodeEntity::file_path.
Asynchronously reads the entire file content into a String using tokio::fs::read_to_string. Handle FileNotFound errors appropriately (map to AppError::FileNotFound). Handle other I/O errors (map to AppError::Io).
Parses the Python source code string using rustpython_parser::parser::parse_program(&source_content, file_path.to_str().unwrap_or_default()).
If parsing fails (returns Err), log the error with file path and details, and return AppError::AstParse { file_path: file_path_str, details: err_string }.
Traverse the resulting rustpython_ast::Suite (the AST root). You will need to write recursive visitor functions or use a visitor pattern if available and suitable.
Crucial - Replicating child.parent = node for context: The Python script dynamically assigns parent attributes. In Rust, this is not idiomatic or safe. Instead, when traversing the AST (e.g., in your recursive visitor functions), you must pass down the necessary parent context (e.g., current ast::StmtKind::ClassDef node's name when visiting its body) as parameters to your traversal functions. This context is needed to determine if a FunctionDef is a "method" or "function", and if an Assign is a class variable or module variable.
For each relevant AST node (FunctionDef, ClassDef, Assign at module/class level), extract information to populate a CodeEntity struct:
entity_type: Determined by node type and parent context (e.g., FunctionDef inside ClassDef is "method").
name: From node.name (for functions/classes) or target.id (for variables).
line_start / line_end: Use node.location.row() and node.end_location.unwrap_or(node.location).row(). rustpython-ast locations are 1-indexed; ensure this matches Python's lineno usage.
docstring: Use rustpython_ast::helpers::get_docstring(node_body) or equivalent logic. This typically involves checking if the first statement in a function/class body is a constant string expression. Default to "" if no docstring.
parent_class: If entity is a method or class variable, store the parent class name.
bases (for ClassDef): Iterate node.bases. For each base, attempt to reconstruct its name (e.g., from ast::ExprKind::Name { id, .. }). If complex, use placeholders like "<complex_base>" or "<error_parsing_base>" as in the Python script ([see Python script extract_code_info for ClassDef bases logic]).
value_repr (for Assign): Replicate Python's simplification. If node.value is ast::ExprKind::Constant { value: ast::Constant::Str(s), .. }, use repr(s). For ast::Constant::Int, ast::Constant::Bool, etc., use their string representation. For List, Tuple, use "[...]". For Dict, use "{...}". Otherwise, default to "<complex_value>". ([see Python script extract_code_info for Assign value_repr logic]).
CRITICAL - signature (for FunctionDef) (Ref: Analysis Doc Sections 2.6, 3.1, 5.2 AST Parsing point 5):
DO NOT ATTEMPT TO REPLICATE THE PYTHON SCRIPT'S _get_signature LINE-MATCHING HEURISTIC. This heuristic is fragile and explicitly identified as a major flaw in the Analysis Document.
Instead, construct a representative signature string using the structural properties of the rustpython_ast::ast::StmtFunctionDef node:
node.name (the function name).
node.args (the arguments). You can iterate through node.args.args and reconstruct a simplified string like (arg1, arg2, *args, **kwargs). If an argument has a type hint (arg.annotation), optionally include it.
node.returns (the return type annotation). Optionally include it.
A robust simplified representation would be def function_name(param1, param2, ...) or def function_name(...). If rustpython-codegen or a similar utility from the rustpython ecosystem can "unparse" or format just the signature part of a FunctionDef node, prefer that. Otherwise, assemble from node.name and a placeholder for arguments like (...). The goal is semantic accuracy based on AST structure, not textual replication of the original (potentially poorly formatted) source lines.
Log any errors encountered while processing a specific AST node within a file but attempt to continue with other nodes in the same file if the error is localized. If a file-level parsing error occurs (e.g., SyntaxError), that file should be skipped as per Python logic.
Return a Vec<CodeEntity> for the file.
X. Redis Operations (Ref: Analysis Doc Sections 2.8, 2.9, 3.2, 5.2 Redis Operations):

Implement in src/redis_ops.rs.

Use the fred crate for all Redis communications.
Client Initialization: In main.rs (or a helper called from there), if mode == OperationMode::Direct, initialize the fred::clients::RedisClient.
Use fred::types::RedisConfig::from_url(&config::get_redis_url())?.
Create client: fred::clients::RedisClient::new(config, None, None, None).
Connect: client.connect().
Wait for connection: client.wait_for_connect().await?.
Handle connection errors and propagate them as AppError::Redis or AppError::Config.
Before any operations in direct mode, PING the server: client.ping().await?.
process_set_command equivalent: Create an async function pub async fn process_set_command(redis_client: Option<&fred::clients::RedisClient>, key: &str, entity_data: &CodeEntity, mode: &crate::cli::OperationMode, key_prefix: &str) -> Result<(), AppError>.
Construct the Redis key string:
If entity_data.entity_type == "method", key is "{key_prefix}:method:{file_path}:{parent_class}.{name}".
Else, key is "{key_prefix}:{entity_type}:{file_path}:{name}". (Match logic from Python [see Python script main loop for key generation]).
generate mode:
Create a temporary map or struct: {"key": constructed_key, "value": entity_data}.
Serialize this map/struct to a JSON string using serde_json::to_string(&command_args_map). This JSON string must be UTF-8 and not escape non-ASCII characters (default for serde_json).
Print to stdout exactly as formatted in the Python script:
```
MCP Tool: redis-myproject / set
Arguments: {json_string_from_above}
---
```

([see Python script process_set_command for generate mode exact output]).
direct mode:
Assert redis_client is Some.
Serialize entity_data (the CodeEntity struct itself) to a JSON string using serde_json::to_string(entity_data).
Execute Redis SET: redis_client.unwrap().set(constructed_key, json_string_of_entity_data, Some(fred::types::Expiration::EX(60 * 60 * 24 * 7)), None, false).await? (Note: Python script does not set TTL, consider if TTL is desired or omit Expiration). For strict Python script parity, omit TTL: redis_client.unwrap().set(constructed_key, json_string_of_entity_data, None, None, false).await?.
Log success/failure.
process_update_set_command equivalent: Create an async function pub async fn process_update_set_command(redis_client: Option<&fred::clients::RedisClient>, set_key: &str, items_to_add: Vec<String>, mode: &crate::cli::OperationMode) -> Result<(), AppError>.
generate mode:
Print to stdout exactly as formatted in the Python script, including all comments and the multi-step agent tasks (get, then multiple sadds).
```json
// Example for get_args JSON: {"key": "your_set_key"}
// Example for sadd_args JSON: {"key": "your_set_key", "member": "item_to_add"}
```

Replicate the loop and println! statements meticulously. ([see Python script process_update_set_command for generate mode exact output]).
direct mode:
Assert redis_client is Some.
If items_to_add is not empty, execute Redis SADD: redis_client.unwrap().sadd(set_key, items_to_add).await?.
Log success/failure.
PROCESSED_FILES_KEY Type Check (Direct Mode Only): Before calling process_update_set_command for PROCESSED_FILES_KEY in direct mode, replicate the Python script's logic:
Get current type of PROCESSED_FILES_KEY: let key_type: String = redis_client.unwrap().type_str(PROCESSED_FILES_KEY).await?;
If key_type != "none" and key_type != "set", log a warning and delete the key: redis_client.unwrap().del(PROCESSED_FILES_KEY).await?; ([see Python script async_main before updating PROCESSED_FILES_KEY]).
XI. Async Orchestration (Ref: Analysis Doc Section 3.4, 5.2 Async Orchestration):

Implement primarily in src/main.rs and src/file_processing.rs.

The main application logic function (e.g., async fn run_indexer(args: CliArgs) -> Result<(), AppError>) will be called from main and be async. Annotate main with #[tokio::main].
File processing loop:
Get list of Python files from file_processing::collect_python_files(&args.app_dir).
Iterate through these files. For each file:
Call ast_parser::extract_entities_from_file.
For each extracted CodeEntity, call redis_ops::process_set_command.
Concurrency (Optional but Recommended by Analysis): Consider using tokio::spawn to process multiple files concurrently. Each spawned task would handle reading, parsing, and then sending its entities (perhaps via a channel, or by calling process_set_command if fred::RedisClient is cloneable and safe for concurrent use – fred clients are designed for this). If using tokio::spawn, ensure shared resources (like RedisClient clone, Arc<Config>) are handled correctly. A Stream of file paths processed concurrently by tokio::task::spawn and buffer_unordered could be efficient.
After processing all files, if any entities were successfully processed and paths collected, call redis_ops::process_update_set_command for config::PROCESSED_FILES_KEY.
Ensure the fred::RedisClient connection is properly closed when done (e.g., client.quit().await? or when it's dropped if fred handles this automatically on drop).
XII. Pythonic Idiom Translation Guidance (Ref: Analysis Doc Section 5.2 Pythonic Idiom):

Python lists of dictionaries (like entities in extract_code_info) become Vec<YourRustStruct> (e.g., Vec<CodeEntity>).
Python's None is represented by Rust's Option<T>. Use #[serde(skip_serializing_if = "Option::is_none")] for optional fields in CodeEntity to omit them from JSON if None, matching Python's json.dumps behavior with missing keys.
Use Rust's format! macro for constructing Redis keys and other formatted strings.
Adhere to Rust's Result<T, AppError> and ? operator for error handling, replacing all Python try-except blocks.
XIII. Adherence to Analysis Document Tables:

You must consult and adhere to the principles and mappings outlined in the following tables from the Analysis Document, which provide high-level guidance on crate choices and idiomatic translations:

Table 1: Grok's Analysis Points - Concurrence and Key Alternatives (Focus on the "Recommended Rust Crates/Approach for Agent Prompt" column).
Table 2: Python Module/Functionality to Rust Crate/Pattern Mapping for Agent Prompt.
XIV. Final Admonitions:

NO DEVIATION: The generated Rust code must strictly adhere to these instructions and the referenced Analysis Document. Do not introduce new architectural patterns or libraries not specified.
EXACTNESS: Pay extreme attention to replicating the exact behavior and, critically, the exact stdout format for generate mode.
ROBUSTNESS: Implement comprehensive error handling as specified.
IDIOMATIC RUST: Write clean, maintainable, and idiomatic Rust code. Use rustfmt (default settings) to format all generated Rust code before finalizing.
COMMENTS: Add comments to explain complex logic or non-obvious decisions that directly stem from these instructions.
COMPLETENESS: Ensure all functionalities of the Python script are ported.
