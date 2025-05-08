# Configuration Management: Redis Code Indexer

## Overview
This document details the configuration strategy for the Rust Code Indexer. All configuration MUST be loaded from a YAML file at `~/.indexer/config.yaml`. No `.env` files or environment variables are supported.

---

## 1. Configuration File Location
- The application MUST look for a YAML config file at:
  - `~/.indexer/config.yaml`
- If the config file is not found, the application MUST use Rust-side hardcoded defaults.
- **No other config sources are permitted.**

---

## 2. YAML Config File Format
- The config file MUST be valid YAML and may include the following keys:

```yaml
redis_url: "redis://localhost:6379"
log_level: "info"
# Add other keys as needed
```

---

## 3. Loading Logic
- On startup, the application MUST:
  1. Expand `~` to the user’s home directory using the `dirs-next` crate.
  2. Attempt to load `~/.indexer/config.yaml` using the `serde_yaml` crate.
  3. If the file exists and is valid YAML, use its values.
  4. If the file does not exist, use hardcoded Rust defaults.
  5. If the file exists but is invalid YAML, print a clear error and exit with nonzero status.

---

## 4. Example Rust Config Struct

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

---

## 5. Security and Best Practices
- Never hardcode sensitive credentials in source files except as safe defaults.
- Do **not** commit `~/.indexer/config.yaml` with secrets to version control.
- If the config file is missing, the application proceeds silently with defaults.
- If the config file is invalid, the application MUST exit with a clear error message.

---

## 6. Troubleshooting
- If configuration is not being loaded, ensure `~/.indexer/config.yaml` exists and is valid YAML.
- For missing or invalid keys, the application will use Rust-side defaults.

