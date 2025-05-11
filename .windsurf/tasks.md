# Project Tasks

This file is the canonical, visible list of tracked tasks for the project. All new tasks, bugfixes, and feature requests should be added here and checked off when complete.

## Open Tasks

### Legacy Tasks (Pre-TDD Workflow)
- [ ] Fix integration test failure for global `--name` parameter in CLI. Current implementation has `--name` defined per subcommand rather than globally as documented in the README.

---

## Vectoring Upgrade (TDD-Driven)

### CLI & Configuration
- [ ] TDD: `vectorize` subcommand structure
  - **Test Spec:** Parsing/validation of required args (`--name`, `--path`), optional overrides (`--provider`, `--db`, `--batch-size`), and utility flags (`--dry-run`, `--verbose`).
  - **Test Spec:** Defaults/overrides logic from config.yaml.
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

