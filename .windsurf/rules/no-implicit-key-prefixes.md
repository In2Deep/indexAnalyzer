---
trigger: always_on
---

All Redis keys must use a prefix of the format code:{project_name} where {project_name} is derived from the basename of the root directory passed via the CLI.