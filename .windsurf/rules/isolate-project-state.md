---
trigger: always_on
---

Indexing a new project must not overwrite or pollute Redis keys from a previously indexed project. Each project must be isolated by its key_prefix.
