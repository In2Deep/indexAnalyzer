---
trigger: always_on
---

Any function returning a Result must propagate the error unless explicitly handled. All parsing failures, Redis errors, and IO errors must result in logs and an error path return.
