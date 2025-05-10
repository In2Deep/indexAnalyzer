---
trigger always_on
---

All Redis keys that include file paths must use POSIXstyle relative paths calculated from the root app directory passed to the CLI No absolute paths or OSspecific slashes may be present in the key
