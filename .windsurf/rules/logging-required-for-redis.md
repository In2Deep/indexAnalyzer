---
trigger: always_on
---

All Redis operations must include appropriate log entries using the existing logging framework. No writes, deletes, or queries should happen silently.
