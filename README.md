# IndexAnalyzer

A high-performance, async codebase indexer and query tool for Python projects, with both Rust and Python CLI implementations. Stores code structure and metadata in Redis for fast recall, search, and analysis.

---

## Table of Contents
- [Features](#features)
- [Requirements](#requirements)
- [Configuration](#configuration)
- [Rust CLI Usage](#rust-cli-usage)
  - [Install](#install)
  - [Commands](#commands)
  - [Examples](#examples)
- [Python CLI Usage](#python-cli-usage)
  - [Install](#install-1)
  - [Commands](#commands-1)
  - [Examples](#examples-1)
- [Development](#development)
- [License](#license)

---

## Features
- Parses Python source code to extract functions, classes, and metadata
- Stores code entities and file info in Redis for fast querying
- Async, modular, and idiomatic code (Rust and Python)
- CLI for indexing, refreshing, recalling, status, and forgetting code
- YAML-based configuration (no env vars or CLI config)

---

## Requirements
- **Redis** (>=6.0) running and accessible (default: `localhost:6379`)
- Python >=3.8 (for Python CLI)
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

> **Note:** The Rust CLI uses positional arguments (not flags) for all commands. See below for usage details. For the Python CLI, see the next section.

### Install
```bash
# In project root
cargo build --release
# Binary will be at target/release/code_indexer_rust
```

### Command Overview
- `remember <project_dir>`: Index all Python files in a project directory
- `refresh <project_dir> <file1.py> [file2.py ...]`: Refresh memory for specific files in a project
- `recall <entity_type> [name] [project_dir]`: Query for code entities (e.g., functions, classes)
- `status <project_dir>`: Show indexed files and project info
- `forget <project_dir>`: Remove all indexed data for a project

### Usage Examples
```bash
# Index a project (add all Python files in the directory to Redis)
./target/release/code_indexer_rust remember ~/shopify_builder/app

# Refresh specific files (provide one or more .py files, space-separated)
./target/release/code_indexer_rust refresh ~/shopify_builder/app foo.py bar.py

# Recall all functions named 'foo' in a project
./target/release/code_indexer_rust recall function foo ~/shopify_builder/app

# Show status for a project
./target/release/code_indexer_rust status ~/shopify_builder/app

# Forget a project (remove all indexed data)
./target/release/code_indexer_rust forget ~/shopify_builder/app
```

---

## Python CLI Usage

> **Note:** The Python CLI uses flag-based arguments (e.g., `--path`, `--project`). The Rust CLI does not. See above for Rust usage.

---

## Python CLI Usage

### Install
```bash
pip install -r requirements.txt
# or
python3 -m venv venv && source venv/bin/activate && pip install -r requirements.txt
```

### Commands
- `remember --path <project_dir>`: Index all Python files in a project
- `refresh --project <project_dir> --files <file1.py,file2.py,...>`: Refresh memory for specific files
- `recall --entity-type <function|class|...> [--name <name>] [--project <dir>]`: Query for code entities
- `status [--project <dir>]`: Show indexed files and project info
- `forget --project <dir>`: Remove all indexed data for a project

### Examples
```bash
# Index a project
python code_indexer.py remember --path ./my_project

# Refresh specific files
python code_indexer.py refresh --project ./my_project --files foo.py,bar.py

# Recall all functions named 'foo'
python code_indexer.py recall --entity-type function --name foo --project ./my_project

# Show status
python code_indexer.py status --project ./my_project

# Forget a project
python code_indexer.py forget --project ./my_project
```

---

## Development
- See `docs/roadmap_part1.md`, `docs/roadmap_part2.md`, `docs/roadmap_part3.md` for feature and implementation details.
- See `docs/dependency_setup.md` for dependency requirements.
- All configuration is YAML-based (see above).

---

## License
MIT (or project-specific, update as needed)
