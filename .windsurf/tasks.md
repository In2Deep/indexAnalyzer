# Project Tasks

This file is the canonical, visible list of tracked tasks for the project. All new tasks, bugfixes, and feature requests should be added here and checked off when complete.

## Open Tasks

### Legacy Tasks (Pre-TDD Workflow)
- [ ] Fix integration test failure for global `--name` parameter in CLI. Current implementation has `--name` defined per subcommand rather than globally as documented in the README.

### Vectoring Upgrade Tasks (Following TDD Workflow)

#### CLI Design & Implementation
- [ ] TDD: `vectorize` subcommand structure
  - **Test Specification:** Test parsing and validation of args: `--name`, `--model`, `--db`, `--batch-size`, `--dry-run`, `--verbose`
  - **RED Phase:** Write failing tests for command structure and argument parsing
  - **GREEN Phase:** Implement minimal code to make tests pass
  - **REFACTOR Phase:** Clean up implementation while keeping tests green

- [ ] TDD: `vector-recall` subcommand structure
  - **Test Specification:** Test parsing and validation of args: `--name`, `--query`, `--top-k`, `--model`, `--db`
  - **RED Phase:** Write failing tests for command structure and argument parsing
  - **GREEN Phase:** Implement minimal code to make tests pass
  - **REFACTOR Phase:** Clean up implementation while keeping tests green

- [ ] Update CLI documentation following documentation-workflow.md after TDD cycles complete

#### Embedding Integration
- [ ] TDD: Abstract embedding logic behind an `Embedder` trait
  - **Test Specification:** Test trait methods (e.g., `generate_embeddings`) using a mock implementation
  - **RED Phase:** Write failing tests for the trait interface
  - **GREEN Phase:** Implement minimal trait definition to make tests pass
  - **REFACTOR Phase:** Clean up trait while keeping tests green

- [ ] TDD: Implement first Embedder backend (OpenAI API)
  - **Test Specification:** Test successful embedding generation and error handling
  - **RED Phase:** Write failing tests for embedding generation
  - **GREEN Phase:** Implement minimal backend to make tests pass
  - **REFACTOR Phase:** Clean up implementation while keeping tests green

- [ ] TDD: Configuration loading for embedding providers
  - **Test Specification:** Test loading config from ~/.indexer/config.yaml and env vars
  - **RED Phase:** Write failing tests for config loading
  - **GREEN Phase:** Implement minimal code to make tests pass
  - **REFACTOR Phase:** Clean up implementation while keeping tests green

#### Vector DB Abstraction
- [ ] TDD: Abstract vector storage/retrieval behind a `VectorStore` trait
  - **Test Specification:** Test trait methods (e.g., `upsert_vectors`, `query_similar_vectors`)
  - **RED Phase:** Write failing tests for the trait interface
  - **GREEN Phase:** Implement minimal trait definition to make tests pass
  - **REFACTOR Phase:** Clean up trait while keeping tests green

- [ ] TDD: Implement Redis VectorStore backend
  - **Test Specification:** Test vector storage, retrieval, and key prefixing
  - **RED Phase:** Write failing tests for Redis vector operations
  - **GREEN Phase:** Implement minimal backend to make tests pass
  - **REFACTOR Phase:** Clean up implementation while keeping tests green

- [ ] TDD: Key prefixing and entity typing enforcement
  - **Test Specification:** Test that all keys use proper project prefixing and entity typing
  - **RED Phase:** Write failing tests for key namespace validation
  - **GREEN Phase:** Implement minimal code to make tests pass
  - **REFACTOR Phase:** Clean up implementation while keeping tests green

#### Vector Indexing Workflow
- [ ] TDD: Entity extraction for vectorization
  - **Test Specification:** Test entity extraction reusing/adapting classic mode logic
  - **RED Phase:** Write failing tests for entity extraction
  - **GREEN Phase:** Implement minimal code to make tests pass
  - **REFACTOR Phase:** Clean up implementation while keeping tests green

- [ ] TDD: Embedding generation for extracted entities
  - **Test Specification:** Test process of taking entities, calling Embedder, receiving vectors
  - **RED Phase:** Write failing tests for embedding generation
  - **GREEN Phase:** Implement minimal code to make tests pass
  - **REFACTOR Phase:** Clean up implementation while keeping tests green

- [ ] TDD: Batch processing for indexing
  - **Test Specification:** Test batching logic and progress logging
  - **RED Phase:** Write failing tests for batch processing
  - **GREEN Phase:** Implement minimal code to make tests pass
  - **REFACTOR Phase:** Clean up implementation while keeping tests green

#### Vector Recall/Search
- [ ] TDD: Similarity search core logic
  - **Test Specification:** Test retrieval of query, calling VectorStore, getting top-K matches
  - **RED Phase:** Write failing tests for similarity search
  - **GREEN Phase:** Implement minimal code to make tests pass
  - **REFACTOR Phase:** Clean up implementation while keeping tests green

- [ ] TDD: Output formatting for recall results
  - **Test Specification:** Test human-readable and machine-parseable output formats
  - **RED Phase:** Write failing tests for output formatting
  - **GREEN Phase:** Implement minimal code to make tests pass
  - **REFACTOR Phase:** Clean up implementation while keeping tests green

## Completed Tasks

- Fix line number extraction in src/ast_parser.rs to use real AST node locations for all entities (function, class, assignment). Add tests and follow all workflows. (commit d4d9958)

- [x] Fix CLI to accept --name as a global argument. The current CLI only accepts positional arguments, so running the application with --name fails. Update argument parsing to support --name as documented, and ensure all commands use it for project namespacing. (Completed in commit 2bbd8a6; note: 1 integration test failure remains for follow-up). in Redis. (commit: c83692b)
