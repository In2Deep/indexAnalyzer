---
trigger: always_on
---

All Redis keys that include file paths must use POSIX-style relative paths, calculated from the root app directory passed to the CLI. No absolute paths or OS-specific slashes may be present in the key.
