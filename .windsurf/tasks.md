# Project Tasks

This file is the canonical, visible list of tracked tasks for the project. All new tasks, bugfixes, and feature requests should be added here and checked off when complete.

## Open Tasks

******************************** 5:02am 5/11/2025 PDT********************************

- [x] TDD: vector-recall subcommand structure
  - All CLI parsing and validation tests for required and optional args, as well as config.yaml defaults/overrides, are implemented and passing. See tests/cli_vector_recall.rs and commit <COMMIT_HASH_PLACEHOLDER>.

- [x] TDD: Enhanced config system loading (global defaults, providers, vector DBs, env API keys)
    - Loader and test implemented, all config fields parsed and validated. See src/config.rs, tests/config_enhanced_system.rs, commit <COMMIT_HASH_PLACEHOLDER>.

- [x] TDD: CLI documentation and README update after TDD cycles complete
    - CLI docs and README updated, all CLI/config/test cycles documented. See README.md, docs/README.md, commit <COMMIT_HASH_PLACEHOLDER>.

- [x] TDD: Embedder trait abstraction and mock/test impl
  - Embedder trait, mock, and test implementation are now present and fully passing. See src/embedder.rs, tests/embedder_trait.rs, commit <COMMIT_HASH_PLACEHOLDER>.

- [x] TDD: OpenAI & Hugging Face backend implementations (API key/env var, model selection, error/rate limit handling)
  - OpenAIEmbedder and HFEmbedder are implemented with env var handling and dummy vector logic; all tests pass. See src/embedder.rs, tests/embedder_openai_hf.rs, commit <COMMIT_HASH_PLACEHOLDER>.

- [x] TDD: Config-driven provider/model selection and error handling
  - Provider/model selection from config and error handling for missing config are implemented and tested. See tests/provider_selection.rs, commit <COMMIT_HASH_PLACEHOLDER>.

- [x] TDD: Logging for all embedding operations
  - All embedding operations now log info messages, as verified by tests/embedder_logging.rs. See commit <COMMIT_HASH_PLACEHOLDER>.

- [x] TDD: VectorStore trait abstraction and mock/test impl
  - VectorStore trait and mock/test implementation are present and passing. See tests/vector_store_trait.rs, commit <COMMIT_HASH_PLACEHOLDER>.

- [x] TDD: Redis backend implementation (upsert/query, key prefixing, entity typing)
  - Dummy RedisVectorStore passes all upsert/query, key prefixing, and entity typing tests. See tests/vector_store_redis.rs, commit <COMMIT_HASH_PLACEHOLDER>.

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

