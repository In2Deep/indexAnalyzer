# workspace-rules-indexanalyzer

these rules are binding for all users and models in this workspace

1. no-free-thinking
- all contributors must follow instructions and docs in docs only
- no creative deviation or unapproved changes

2. documentation-is-law
- only roadmap-part-md dependency-setup-md configuration-md are source of truth
- if not in docs do not implement

3. only-approved-libraries
- use only crate versions and libraries in dependency-setup-md
- no deprecated or experimental dependencies

4. yaml-configuration-only
- all config must load from home-indexer-config-yaml
- no env no environment variables no cli config

5. async-modular-idiomatic-rust
- all io and network ops must be async
- code must match module structure in roadmap

6. no-stubs-no-todos-no-future-work
- every feature in roadmap must be implemented and tested
- no partials or placeholders allowed

7. mandatory-testing
- all code must include async unit and integration tests
- code without tests is incomplete

8. output-compliance
- generate mode output must match python script format exactly

9. error-handling-and-logging
- error handling and logging must follow roadmap
- no panics unwraps or silent failures except on startup

10. file-and-directory-hygiene
- do not modify files outside src docs unless required
- do not track or modify target or build artifacts

11. model-user-handoff
- before handoff ensure code tests and docs are up to date and match roadmap
