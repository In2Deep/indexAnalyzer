# indexer — Detailed Documentation

This document expands on the main project `README.md` and provides in-depth details for advanced users and developers.

---

## Table of Contents
- [Overview](#overview)
- [Architecture](#architecture)
- [Configuration](#configuration)
- [Commands & Usage](#commands--usage)
- [Rust CLI Details](#rust-cli-details)

- [Redis Schema](#redis-schema)
- [Troubleshooting](#troubleshooting)
- [Development & Roadmap](#development--roadmap)
- [License](#license)

---

## Overview
indexer is a cross-language codebase indexer and query tool. It parses source code, extracts code entities, and stores metadata in Redis for fast recall and search. It is implemented in Rust (async, high-performance), which serves as the canonical implementation.

---

## Architecture
- **Rust CLI**: Async, modular, high performance. Uses `fred 10.1.x` for Redis.

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
All commands are available in the Rust CLI:
- `remember`: Index all Python files in a project
- `refresh`: Refresh memory for specific files
- `recall`: Query for code entities (functions, classes, etc.)
- `status`: Show indexed files and project info
- `forget`: Remove all indexed data for a project

### Project Namespacing with --name
- Use `--name <project>` (or `--project-name <project>`) to specify the project name for Redis key prefixing.
- If not provided, the project name defaults to the last segment of the target directory.
- All Redis keys are namespaced as `code_index:<project>:...` to ensure project isolation.

**Example:**
```bash
./target/release/indexer remember --name myproject --path ~/myproject
```

---

## Rust CLI Details
- **Build:** `cargo build --release`
- **Run:** `./target/release/indexer <command> [options]`
- **Dependencies:** See `../docs/dependency_setup.md`
- **Testing:** `cargo test --all -- --nocapture`
- **Error Handling:** All errors are logged; no panics except on startup.
- **Async:** All operations are non-blocking and async.

---

## Redis Schema
- `code_index:file_index` — Set of indexed file paths
- `code_index:<rel_path>` — File metadata (JSON)
- `code_index:entity:<type>:<name>` — Code entity metadata (JSON)
- Project and entity keys are prefixed by `key_prefix` (default: `code_index`)

---

## Troubleshooting
- **Redis connection errors:** Ensure Redis is running and accessible at the configured URL using `fred 10.1.0`.
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
