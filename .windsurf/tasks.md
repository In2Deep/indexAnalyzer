# Project Tasks

This file is the canonical, visible list of tracked tasks for the project. All new tasks, bugfixes, and feature requests should be added here and checked off when complete.

## Open Tasks

******************************** 11:40pm 5/10/2025 PDT********************************

- [ ] TDD: Enhanced Configuration System – Debugging and Completing Config Loader Robustness

    **Current Status (2025-05-10 23:39 PDT):**

    1. We are in the midst of a strict TDD cycle focused on making the configuration loader (`AppConfig::load` in `src/config.rs`) robust to all error conditions, with a focus on correct error propagation and test isolation.

    2. The following tests in `tests/config_enhanced.rs` are under active development and debugging:
       - `test_load_config_with_global_defaults` (PASSING)
       - `test_load_config_from_file` (FAILING: HomeDirNotFound)
       - `test_load_config_bad_yaml` (FAILING: HomeDirNotFound)
       - `test_load_config_missing_home` (PASSING)

    3. The primary technical blocker is that, despite setting the `HOME` environment variable to a valid temporary directory (created with `tempfile::tempdir()`), the config loader still returns `ConfigError::HomeDirNotFound` in both the 'from_file' and 'bad_yaml' tests. This indicates either:
       - The temp directory is not being recognized as the home directory within the test process.
       - The directory is being dropped or removed before `AppConfig::load` is called.
       - There may be platform-specific issues with how Rust and the OS handle temporary directories and environment variables in test contexts.

    4. We have:
       - Ensured that the temp dir and the `HOME` variable persist for the entire scope of the test (no early drops).
       - Added debug output to print the value of `HOME`, whether the directory exists, and its contents immediately before calling `AppConfig::load`.
       - Avoided all hardcoded paths or logic, per project rules.

    5. The next step is to fix a compilation error in the debug output (caused by attempting to use `.unwrap_or_default()` on a `Result<Vec<PathBuf>, std::io::Error>`, which is not valid). The debug output will be adjusted to print either the directory contents or the error, as appropriate.

    6. Once debug output is working, we will rerun the tests to:
       - Confirm that the temp home directory exists and is accessible at the time of the config load.
       - Confirm that the config file is present and readable.
       - If the loader still returns `HomeDirNotFound`, we will further investigate how the loader resolves the home directory and if there are any platform-specific behaviors.

    7. The overall goal is to:
       - Ensure that `AppConfig::load` only returns `ConfigError::HomeDirNotFound` when `HOME` is truly unset or inaccessible.
       - Ensure that if a config file exists but is malformed, `ConfigError::Yaml` is returned.
       - Ensure that if a valid config file exists, its values are loaded and override defaults.
       - Guarantee that all config tests are fully isolated and leave no side effects between runs.

    8. **Recent changes include:**
       - Removing all fallback logic to `dirs_next::home_dir` in favor of strictly using `std::env::var("HOME")` for home detection.
       - Ensuring every test sets and cleans up its own `HOME` and config files.
       - Refactoring test scoping to keep temp directories alive for the full test duration.
       - Removing unused imports and warnings from the codebase.

    9. **Next steps for resuming work:**
       - Fix debug print compilation error in `test_load_config_from_file`.
       - Rerun the test and analyze debug output.
       - If the temp dir is present and accessible, but the loader still fails, investigate how the config loader is resolving the home path.
       - Once the root cause is identified, make the minimal code changes needed to get all config tests green.
       - Document all changes and update the README and roadmap as needed.

    10. **General handoff context:**
        - All work is being done in strict accordance with TDD, zero-warnings, and project isolation rules.
        - No implementation code is being written before a failing test is confirmed.
        - All config loader changes are being tracked in .windsurf/tasks.md and referenced in commit messages.
        - All test and loader logic is being kept as minimal and idiomatic as possible, with no hardcoded logic or platform-specific hacks.
        - The next session should begin by fixing the debug print, running the test, and continuing the TDD cycle until all config tests pass.


## Completed Tasks

- [x] TDD: select_provider_id returns error if no provider specified in CLI or config
  - Implemented error return for missing provider in select_provider_id (test_provider_error_if_none_and_config_empty). See commit <COMMIT_HASH_PLACEHOLDER>.


- [x] TDD: select_provider_id uses CLI value when --provider is specified
  - Added and passed unit test for CLI override (test_provider_cli_overrides_config). See commit <COMMIT_HASH_PLACEHOLDER>.


- [x] Implement provider selection fallback for vectorize subcommand
  - Implemented select_provider_id and passing test (test_provider_fallback_to_config_default) in strict TDD cycle. See commit <COMMIT_HASH_PLACEHOLDER>.

### Legacy Tasks (Pre-TDD Workflow)
- [ ] Fix integration test failure for global `--name` parameter in CLI. Current implementation has `--name` defined per subcommand rather than globally as documented in the README.

---

## Vectoring Upgrade (TDD-Driven)

### CLI & Configuration
- [x] TDD: vectorize subcommand parses mandatory args (`--name`, `--path`). (R-G-R complete, test: test_vectorize_parsing_mandatory_args)
- [x] TDD: vectorize subcommand parses optional override arg (`--provider`).
  - Parsing and validation covered by test_vectorize_parsing_provider_arg (present and absent cases). See commit <COMMIT_HASH_PLACEHOLDER>.

- [ ] TDD: `vector-recall` subcommand structure
  - **Test Spec:** Parsing/validation of required args (`--name`, `--query`), optional overrides (`--provider`, `--db`, `--top-k`), and utility flag (`--json`).
  - **Test Spec:** Defaults/overrides logic from config.yaml.
- [ ] TDD: Enhanced config system loading (global defaults, providers, vector DBs, env API keys).
- [ ] TDD: CLI documentation and README update after TDD cycles complete.

### Embedding Integration
- [ ] TDD: `Embedder` trait abstraction and mock/test impl.
- [ ] TDD: OpenAI & Hugging Face backend implementations (API key/env var, model selection, error/rate limit handling).
- [ ] TDD: Config-driven provider/model selection and error handling.
- [ ] TDD: Logging for all embedding operations.

### Vector DB Abstraction
- [ ] TDD: `VectorStore` trait abstraction and mock/test impl.
- [ ] TDD: Redis backend implementation (upsert/query, key prefixing, entity typing).
- [ ] TDD: Logging for all Redis/vector DB operations.

### Vector Indexing Workflow
- [ ] TDD: Entity extraction for vectorization (reuse/adapt classic logic, verify no side-effects).
- [ ] TDD: Embedding generation for extracted entities (call Embedder, receive vectors, error handling).
- [ ] TDD: Batch processing and progress logging.

### Vector Recall/Search
- [ ] TDD: Similarity search logic (query, call VectorStore, get top-K, error handling).
- [ ] TDD: Output formatting (human-readable, JSON flag).
- [ ] TDD: Logging of all query parameters/results.

### Migration & Compatibility
- [ ] TDD: Classic/vector data coexistence (test key isolation, no mixing/overwrites).
- [ ] TDD: Document migration instructions if schema changes are required.

### Testing & Documentation
- [ ] TDD: Coverage tracking, placeholder tests for unimplemented behaviors.
- [ ] TDD: Update all docs and usage examples after each TDD cycle.

---

## Completed Tasks

- Fix line number extraction in src/ast_parser.rs to use real AST node locations for all entities (function, class, assignment). Add tests and follow all workflows. (commit d4d9958)

- [x] Fix CLI to accept --name as a global argument. The current CLI only accepts positional arguments, so running the application with --name fails. Update argument parsing to support --name as documented, and ensure all commands use it for project namespacing. (Completed in commit 2bbd8a6; note: 1 integration test failure remains for follow-up). in Redis. (commit: c83692b)

