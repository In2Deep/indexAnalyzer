---
trigger: always_on
---

All configuration must be loaded from ~/.indexer/config.yaml, except for sensitive values (such as API keys), which must be loaded from environment variables first. If not found in the environment, fall back to ~/.indexer/config.yaml. No hardcoding of secrets or use of alternative config sources is permitted.