# Roadmap: Vectoring Upgrade for Code Indexer

## 1. Objectives
- Add robust vector-based indexing and search to the code indexer, supporting code entity embeddings and similarity queries.
- Maintain strict separation between classic and vector workflows—no cross-contamination of logic or keys.
- Preserve and document all existing CLI functionality and workflows.
- Ensure all changes are modular, fully async, and warning-free.

## 2. Requirements
- **CLI:** New `vectorize` subcommand for vector indexing, and `vector-recall` for similarity search.
  - Arguments: embedding model, vector DB backend, batch size, dry-run, verbose, etc.
- **Embedding:** Pluggable support for at least one embedding provider (OpenAI, Hugging Face, or local model), with config in `~/.indexer/config.yaml`.
- **Vector DB:** Store embeddings in a configurable vector database (Redis, Qdrant, Pinecone, etc.), defaulting to Redis.
- **Recall:** Implement similarity search over stored vectors, with CLI output matching recall conventions.
- **Testing:** Comprehensive async tests for all new logic, covering success and failure paths.
- **Docs:** Full documentation and usage examples for vector workflows, config, and migration.

## 3. Design Constraints
- **Async Workflow:** All code must follow the `test-and-commit-after-change.md` workflow.
- **Documentation Workflow:** All code must follow the `documentation-workflow.md` workflow.
- **Mandatory Testing:** All code must follow the `mandatory-testing.md` workflow.
- **No Creative Output:** All code must follow the `no-creative-output.md` workflow.
- **No Free Thinking:** All code must follow the `no-free-thinking.md` workflow.
- **No Stubs or TODOs:** All code must follow the `no-stubs-no-todos-no-future-work.md` workflow.
- **Project Structure:** No architectural deviation or new modules/files unless explicitly added to roadmap and README.
- **Dependencies:** No new crates without explicit approval and README update.
- **Async Discipline:** All code must be async and idiomatic Rust (Tokio, async traits, etc.).
- **Zero Warnings:** All code must compile and test with zero warnings/errors at all times.
- **Error Handling:** All errors must be logged and surfaced per project standards—no silent failures.
- **Key Hygiene:** Vector keys must use proper namespacing and never pollute classic keys.

## 4. Implementation Plan (Detailed)
### 4.1 CLI Design
- Add `vectorize` subcommand to CLI parser (`cli.rs`).
  - Args: `--name`, `--model`, `--db`, `--batch-size`, `--dry-run`, `--verbose`.
- Add `vector-recall` subcommand for similarity search.
  - Args: `--name`, `--query`, `--top-k`, `--model`, `--db`.
- Update CLI docs and README accordingly.

### 4.2 Embedding Integration
- Abstract embedding logic behind a trait (e.g., `Embedder`).
- Implement at least one backend (OpenAI API, or local model if preferred).
- Read embedding config from `~/.indexer/config.yaml` or env.
- Handle rate limits, errors, and retries with proper logging.

### 4.3 Vector DB Abstraction
- Abstract vector storage/retrieval behind a trait (e.g., `VectorStore`).
- Implement Redis backend first; plan for Qdrant/Pinecone as future options.
- Ensure all keys use strict project prefixing and entity typing.
- Add logging for all Redis/vector DB operations.

### 4.4 Vector Indexing Workflow
- Extract entities as in classic mode.
- For each entity, generate embedding and store in vector DB with metadata (file, entity type, signature, etc.).
- Support batch processing and progress logging.
- On error, log and continue (unless fatal).

### 4.5 Vector Recall/Search
- Implement similarity search CLI, returning top-K matches with metadata.
- Output must be human-readable and optionally machine-parseable (e.g., JSON flag).
- Log all query parameters and results.

### 4.6 Migration & Compatibility
- Classic and vector data must coexist; never overwrite or mix keys.
- Provide migration instructions if schema changes are required.

### 4.7 Testing
- For every function and logic block, add or expand async tests (success and failure cases).
- No commit allowed unless all tests pass and zero warnings.
- If no test exists, generate a skeleton and halt until implemented.

### 4.8 Documentation
- Update all READMEs and usage docs for new commands and config.
- Add migration notes and troubleshooting for vector features.
- Document test coverage and known limitations.

## 5. Forbidden Actions
- No changes to classic indexing unless required for compatibility (must be documented).
- No stubs, TODOs, or incomplete code.
- No silent errors, warning suppression, or unlogged failures.
- No creative output or deviation from this roadmap unless explicitly approved and documented.

## 6. Milestones
- [ ] CLI updated with vector commands and fully documented
- [ ] Embedding integration complete and tested
- [ ] Vector DB abstraction complete and tested
- [ ] End-to-end vector indexing workflow working
- [ ] Vector recall/search working and tested
- [ ] All tests passing, zero warnings
- [ ] Documentation and migration notes complete

---

*All changes must reference this roadmap and be tracked in .windsurf/tasks.md. No handoff or merge unless every milestone is green and warning-free.*
