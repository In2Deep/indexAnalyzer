# Project Tasks

This file is the canonical, visible list of tracked tasks for the project. All new tasks, bugfixes, and feature requests should be added here and checked off when complete.

## Open Tasks

******************************** 5:02am 5/11/2025 PDT********************************

- [x] TDD: vector-recall subcommand structure
  - All CLI parsing and validation tests for required and optional args, as well as config.yaml defaults/overrides, are implemented and passing. See tests/cli_vector_recall.rs and commit <COMMIT_HASH_PLACEHOLDER>.

- [ ] TDD: Enhanced config system loading (global defaults, providers, vector DBs, env API keys)
    - **Test Spec:** Write a test to verify that the loader can parse and return global defaults, provider blocks, and vector DB configs from config.yaml. Test should fail until implemented.

- [ ] TDD: CLI documentation and README update after TDD cycles complete
    - **Test Spec:** Write a test or script to verify that CLI documentation and README are updated to match the current CLI structure and config logic. Test should fail until implemented.

- [ ] TDD: Embedder trait abstraction and mock/test impl
    - **Test Spec:** Write a test to verify that the Embedder trait is defined and a mock implementation can be used in tests. Test should fail until implemented.

- [ ] TDD: OpenAI & Hugging Face backend implementations (API key/env var, model selection, error/rate limit handling)
    - **Test Spec:** Write a test to verify that the Embedder can call OpenAI and Hugging Face APIs with the correct keys and model selection, and handles errors/rate limits. Test should fail until implemented.

- [ ] TDD: Config-driven provider/model selection and error handling
    - **Test Spec:** Write a test to verify that provider/model selection is driven by config and proper errors are returned for missing/invalid config. Test should fail until implemented.

- [ ] TDD: Logging for all embedding operations
    - **Test Spec:** Write a test to verify that all embedding operations produce appropriate log output, including errors and API calls. Test should fail until implemented.

- [ ] TDD: VectorStore trait abstraction and mock/test impl
    - **Test Spec:** Write a test to verify that the VectorStore trait is defined and a mock implementation can be used in tests. Test should fail until implemented.

- [ ] TDD: Redis backend implementation (upsert/query, key prefixing, entity typing)
    - **Test Spec:** Write a test to verify that the Redis backend can upsert/query vectors, uses correct key prefixing, and supports entity typing. Test should fail until implemented.

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

