# IndexAnalyzer — Detailed Documentation

This document expands on the main project `README.md` and provides in-depth details for advanced users and developers.

---

## Table of Contents
- [Overview](#overview)
- [Architecture](#architecture)
- [Configuration](#configuration)
- [Commands & Usage](#commands--usage)
- [Rust CLI Details](#rust-cli-details)
- [Python CLI Details](#python-cli-details)
- [Redis Schema](#redis-schema)
- [Troubleshooting](#troubleshooting)
- [Development & Roadmap](#development--roadmap)
- [License](#license)

---

## Overview
IndexAnalyzer is a cross-language codebase indexer and query tool. It parses Python source code, extracts code entities, and stores metadata in Redis for fast recall and search. It is implemented in both Rust (async, high-performance) and Python (reference/compatibility).

---

## Architecture
- **Rust CLI**: Async, modular, high performance. Uses `fred` for Redis, `rustpython_parser` for AST.
- **Python CLI**: Async, feature-complete, reference implementation.
- **Redis**: Central store for code entities, file index, and project metadata.
- **Config**: YAML only, loaded from `~/.indexer/config.yaml`.

---

## Configuration
See `../docs/configuration.md` for all keys and options. Example:
```yaml
redis_url: "redis://127.0.0.1:6379/0"
log_level: "info"
```

---

## Commands & Usage
All commands are available in both CLIs:
- `remember`: Index all Python files in a project
- `refresh`: Refresh memory for specific files
- `recall`: Query for code entities (functions, classes, etc.)
- `status`: Show indexed files and project info
- `forget`: Remove all indexed data for a project

See the main `README.md` for CLI argument details and examples.

---

## Rust CLI Details
- **Build:** `cargo build --release`
- **Run:** `./target/release/code_indexer_rust <command> [options]`
- **Dependencies:** See `../docs/dependency_setup.md`
- **Testing:** `cargo test --all -- --nocapture`
- **Error Handling:** All errors are logged; no panics except on startup.
- **Async:** All operations are non-blocking and async.

---

## Python CLI Details
- **Run:** `python code_indexer.py <command> [options]`
- **Dependencies:** See `requirements.txt`
- **Async:** Uses `asyncio` for all I/O and Redis operations.

---

## Redis Schema
- `code_index:file_index` — Set of indexed file paths
- `code_index:<rel_path>` — File metadata (JSON)
- `code_index:entity:<type>:<name>` — Code entity metadata (JSON)
- Project and entity keys are prefixed by `key_prefix` (default: `code_index`)

---

## Troubleshooting
- **Redis connection errors:** Ensure Redis is running and accessible at the configured URL.
- **No entities found:** Check that the project path is correct and files are Python source files.
- **Config not loading:** Ensure `~/.indexer/config.yaml` exists and is valid YAML.

---

## Development & Roadmap
- See `roadmap_part1.md`, `roadmap_part2.md`, `roadmap_part3.md` for details.
- Only libraries and versions in `dependency_setup.md` are allowed.
- All code must be async, modular, and tested.

---

## License
MIT (or project-specific, update as needed)
