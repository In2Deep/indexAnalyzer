---
trigger: always_on
---

- Before writing any Rust code, **read and understand the logic and intent of the Python `code_indexer.py`**. Do NOT translate line-by-line—fully parse and comprehend the purpose and flow.
- Implement features module-by-module as described in the roadmap, with no deviation.
- All code must be async, modular, and idiomatic.
- Use only the libraries and versions specified in `dependency_setup.md`.
- All configuration must be loaded from `~/.indexer/config.yaml`.