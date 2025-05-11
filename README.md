# indexer

A high-performance, async codebase indexer and query tool for Python projects, implemented in Rust. Stores code structure and metadata in Redis for fast recall, search, and analysis.

---

## Table of Contents
- [Features](#features)
- [Requirements](#requirements)
- [Configuration](#configuration)
- [Rust CLI Usage](#rust-cli-usage)
  - [Install](#install)
  - [Commands](#commands)
  - [Examples](#examples)

- [Development](#development)
- [License](#license)

---

## Features
- Parses Python source code to extract functions, classes, and metadata
- Stores code entities and file info in Redis for fast querying
- Async, modular, and idiomatic Rust code
- CLI for indexing, refreshing, recalling, status, and forgetting code
- YAML-based configuration (no env vars or CLI config)

---

## Requirements
- **Redis** (>=6.0) running and accessible (default: `localhost:6379`)

- Rust (stable, see `docs/dependency_setup.md` for required toolchain)

---

## Configuration
All configuration is loaded from `~/.indexer/config.yaml`.

Example `config.yaml`:
```yaml
redis_url: "redis://127.0.0.1:6379/0"
log_level: "info"
```
See `docs/configuration.md` for all options.

---

## Rust CLI Usage

> **Note:** The Rust CLI uses the `--name` (or `--project-name`) argument to specify the project name for Redis key prefixing. This parameter is a finalized part of the interface and will not change. All Redis keys are namespaced as `code_index:<project>:...` to ensure project isolation.

### Install
```bash
# In project root
cargo build --release
# Binary will be at target/release/indexer
```

### Command Overview
- `remember --name <project> --path <project_dir>`: Index all Python files in a project directory
- `refresh --name <project> --files <file1.py,file2.py,...>`: Refresh memory for specific files in a project
- `recall <entity_type> [name] --name <project>`: Query for code entities (e.g., functions, classes)
- `status --name <project>`: Show indexed files and project info
- `forget --name <project>`: Remove all indexed data for a project

### Usage Examples
```bash
# Index a project with a specific project name
./target/release/indexer remember --name my_project --path ~/my_project

# Refresh specific files by name
./target/release/indexer refresh --name my_project --files foo.py,bar.py

# Recall all functions named 'foo' in a project
./target/release/indexer recall function foo --name my_project

# Show status for a project
./target/release/indexer status --name my_project

# Forget (remove) all indexed data for a project
./target/release/indexer forget --name my_project
```

### Commands
- `remember --path <project_dir>`: Index all Python files in a project
- `refresh --project <project_dir> --files <file1.py,file2.py,...>`: Refresh memory for specific files
- `recall --entity-type <function|class|...> [--name <name>] [--project <dir>]`: Query for code entities
- `status [--project <dir>]`: Show indexed files and project info
- `forget --project <dir>`: Remove all indexed data for a project
- `vectorize --name <project> --model <provider> --db <backend> [--batch-size <N>] [--dry-run] [--verbose]`: Generate and index code embeddings for a project
- `vector-recall --name <project> --query <text> [--top-k <N>] [--model <provider>] [--db <backend>] [--json]`: Semantic similarity search over indexed code entities

### Vectorization & Recall (Vector Features)

Vector-based indexing and search are now supported via two subcommands:
- `vectorize`: Batch-generate embeddings for code entities and store them in a vector database (default: Redis). Supports pluggable embedding providers (OpenAI, Hugging Face, OpenRouter).
- `vector-recall`: Perform similarity search over indexed code using a query string.

#### `vectorize` Arguments
- `--name <project>`: Project name for namespacing embeddings
- `--model <provider>`: Embedding provider (e.g., openai, huggingface, openrouter)
- `--db <backend>`: Vector DB backend (e.g., redis)
- `--batch-size <N>`: Batch size for indexing (optional)
- `--dry-run`: Show what would be indexed, but do not write to DB (optional)
- `--verbose`: Extra logging (optional)

#### `vector-recall` Arguments
- `--name <project>`: Project name for namespacing
- `--query <text>`: Query string for similarity search
- `--top-k <N>`: Number of results to return (optional)
- `--model <provider>`: Embedding provider to use for query (optional)
- `--db <backend>`: Vector DB backend (optional)
- `--json`: Output results in machine-readable JSON (optional)

#### Example Usage
```bash
# Vectorize a project with OpenAI embeddings, storing in Redis
./target/release/indexer vectorize --name my_project --model openai --db redis --batch-size 100 --verbose

# Semantic search for similar code entities
./target/release/indexer vector-recall --name my_project --query "tokenize text" --top-k 5 --json
```

---

### Configuration for Vector Features
All configuration is loaded from `~/.indexer/config.yaml`.

Example vector config:
```yaml
redis_url: "redis://127.0.0.1:6379/0"
log_level: "info"
global_defaults:
  provider: "openai"
  db: "redis"
providers:
  openai:
    api_key: "${OPENAI_API_KEY}"
    model: "text-embedding-ada-002"
vector_dbs:
  redis:
    url: "redis://127.0.0.1:6379/0"
    key_prefix: "code:myproject"
```
See `docs/configuration.md` for all options.

---

### Output Formatting & Logging
- Use `--json` with `vector-recall` for machine-readable output; default is human-readable.
- All embedding and vector DB operations are logged per project standards.
- See `.windsurf/tasks.md` and `docs/roadmap.md` for TDD status, test coverage, and development methodology.

---

### Test-Driven Development (TDD) & Documentation
- All new features are developed using strict TDD: RED test, GREEN code, refactor, document, repeat.
- See `.windsurf/tasks.md` and `docs/roadmap.md` for up-to-date task tracking and roadmap.

> **Development Note:** All new vector features are implemented using a strict TDD workflow. See `docs/roadmap.md` and `.windsurf/tasks.md` for development details and status.

All usage and examples below refer to the Rust implementation only.
---

## Development

### TDD Progress (2025-05-10)
- All new features, including `vectorize` and vector search, are implemented via strict Red-Green-Refactor TDD cycles.
- **CLI parsing for `vectorize`** (mandatory/optional args) is fully tested and stable (see `tests/cli_vectorize.rs`).
- **Provider selection fallback logic**: If `--provider` is not specified, the default from config is selected (see `tests/vectorize_logic.rs`).
- All TDD tasks and feature status are tracked in `.windsurf/tasks.md`.
- See also: `docs/README.md` for advanced details.

- See `docs/roadmap_part1.md`, `docs/roadmap_part2.md`, `docs/roadmap_part3.md` for feature and implementation details.
- See `docs/dependency_setup.md` for dependency requirements.
- All configuration is YAML-based (see above).

---

## License
MIT (or project-specific, update as needed)
