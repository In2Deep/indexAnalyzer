# Dependency Setup: Redis Code Indexer

## Overview
This document details the process of setting up and managing dependencies for the Redis Code Indexer, including version selection and compatibility notes. Follow these instructions to ensure a reproducible and robust development environment.

---

## 1. Add Dependencies to Cargo.toml (Latest Stable, May 2025)

Use only the following libraries and versions (latest stable as of May 2025):

```toml
[dependencies]
tokio = { version = "1.37", features = ["full"] }           # Async runtime
fred = { version = "10.0.0", features = ["serde_json"] } # Redis client (async initialization: see roadmap_part1.md for usage pattern)
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
serde_yaml = "0.9"                                            # For YAML config parsing
ignore = "0.4"
rustpython-ast = "0.4.0"                                     # Python AST parsing
log = "0.4"
fern = "0.6"
once_cell = "1.19"
dirs-next = "2.0"                                            # Home directory resolution
clap = { version = "4.5", features = ["derive"] }
thiserror = "1.0"
```

**DO NOT** use any crate not listed above. **DO NOT** change features or versions unless a newer, stable release is available and approved.

---

## 2. Rationale for Version Choices
- Other libraries are kept at their latest stable versions to benefit from security and performance improvements.

---

## 3. Installation Steps
1. Update `Cargo.toml` with the specified dependencies and versions.
2. Run `cargo fetch` or `cargo build` to download and build dependencies.
3. If using Python AST parsing, ensure the selected parser crate is compatible with your Rust toolchain.

---

## 4. Compatibility Notes
- Carefully review breaking changes in Fred and other libraries before upgrading.
- Use `cargo update` cautiously to avoid unintentional upgrades.
- Document any changes in dependency versions in this file for future reference.
