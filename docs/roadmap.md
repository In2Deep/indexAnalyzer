# Roadmap: Vectoring Upgrade for Code Indexer (TDD-Driven)

## 1. Objectives
- Add robust vector-based indexing and search to the code indexer, supporting code entity embeddings and similarity queries.
- Maintain strict separation between classic and vector workflows—no cross-contamination of logic or keys.
- Preserve and document all existing CLI functionality and workflows.
- Ensure all changes are modular, fully async, and warning-free, developed using a strict Test-Driven Development methodology.

## 2. Requirements
- CLI: New vectorize and vector-recall subcommands. 
  - Core arguments e.g. project name path or query will be required on the CLI.
  - Detailed configurations for embedding providers models vector DBs API key sourcing and operational defaults e.g. batch size top-k will be managed via an expanded ~/.indexer/config.yaml.
  - CLI flags will be available for optional overrides of these configured defaults.
  - Standard utility flags e.g. --dry-run --verbose --json will remain as optional CLI flags.
- Embedding: Pluggable support for OpenAI, Hugging Face and OpenRouter
- Vector DB: Store embeddings in a configurable vector database Redis
- Recall: Implement similarity search over stored vectors, with CLI output matching recall conventions.
- Testing: All new logic will be developed following the /tdd-workflow-feature-development-cycle, ensuring comprehensive async tests covering success and failure paths for every unit of behavior.
- Docs: Full documentation and usage examples for vector workflows, config, and migration, generated alongside development.

## 3. Design Constraints
- Core Development Methodology: All new feature development and significant modifications outlined in this roadmap MUST strictly adhere to the /tdd-workflow-feature-development-cycle.
- Async Workflow: All code must follow the /test-and-commit-after-change workflow which will be synergistic with the TDD cycle commits.
- Documentation Workflow: All code must follow the /documentation-workflow.
- Mandatory Testing via TDD: The @mandatory-testing principles are fulfilled and superseded by the comprehensive nature of the /tdd-workflow-feature-development-cycle.
- No Creative Output: All code must follow the @no-creative-output workflow; minimal code to pass tests is paramount in the GREEN phase of TDD.
- No Free Thinking: All code must follow the @no-free-thinking.
- No Stubs or TODOs: All code must follow the @no-stubs-no-todos-no-future-work.
- Project Structure: No architectural deviation or new modules/files unless explicitly added to roadmap, README, and first defined by failing tests.
- Dependencies: No new crates without explicit approval, README update, and consideration for testability.
- Async Discipline: All code must be async and idiomatic Rust Tokio, async traits, etc. adhering to @async-modular-idiomatic-rust.
- Zero Warnings: All code must compile and test with zero warnings/errors at all times, verified at each step of the TDD cycle, as per @zero-warnings-required.
- Error Handling: All errors must be logged and surfaced per project standards—no silent failures; error paths MUST be tested, following @error-handling-and-logging and @no-silent-errors.
- Key Hygiene: Vector keys must use proper namespacing and never pollute classic keys; test cases must verify this, following @isolate-project-state and @enforce-consistent-key-prefix.

## 4. Implementation Plan (TDD-Driven)

Overarching Process Note: Each item below represents a feature or a set of related behaviors. Development for every item will proceed by:
1.  Creating/updating a specific task in .windsurf/tasks.md (as per @task-tracking).
2.  Strictly following all phases of the /tdd-workflow-feature-development-cycle:
    - Phase 1: Task Definition & Test Specification.
    - Phase 2: Write Failing Test (RED).
    - Phase 3: Write Minimal Code to Pass (GREEN).
    - Phase 4: Refactor.
    - Phase 5: Cycle Completion.
3.  Adhering to all relevant Design Constraints and rules throughout the cycle.

### 4.1 CLI Design & Configuration System Implementation
-   Feature: vectorize subcommand structure and argument parsing (cli.rs).
    -   Define behavior for mandatory args: --name, --path.
    -   Define behavior for optional override args: [--provider an_id_from_config], [--db an_id_from_config], [--batch-size N].
    -   Define behavior for utility args: --dry-run, --verbose.
    -   TDD parsing of these args and logic for defaulting to settings from config.yaml when optional args are omitted.
-   Feature: vector-recall subcommand structure and argument parsing.
    -   Define behavior for mandatory args: --name, --query.
    -   Define behavior for optional override args: [--provider an_id_from_config], [--db an_id_from_config], [--top-k N].
    -   Define behavior for utility args: --json.
    -   TDD parsing of these args and logic for defaulting to settings from config.yaml when optional args are omitted.
-   Feature: Enhanced Configuration System based on ~/.indexer/config.yaml.
    -   TDD loading of global defaults: default_embedding_provider_id, default_vector_db_id, default_batch_size, default_top_k.
    -   TDD loading of a list of embedding_providers sections each potentially containing: id, type, api_key_env_var for API key retrieval, default_model.
    -   TDD loading of a list of vector_dbs sections each potentially containing: id, type, url, and other backend-specific connection parameters.
    -   TDD core application logic to correctly:
        -   Utilize global defaults when no relevant CLI overrides or specific provider or DB settings are present.
        -   Select embedding provider and vector DB configurations based on ID specified by CLI override or global default.
        -   Use a provider specific default_model when a provider is selected but no specific model is overridden for it.
        -   Retrieve API keys by looking up the environment variable name specified in the selected embedding_provider configuration.
-   Task: Update CLI docs and README following /documentation-workflow after TDD cycles complete for CLI structure and configuration system.

### 4.2 Embedding Integration
-   Feature: Abstract embedding logic behind an Embedder trait.
    -   Define trait methods e.g., generate_embeddings(texts: Vec<String>) -> Result<Vec<EmbeddingVector>, Error>.
    -   TDD the trait definition by testing a mock/stub implementation if necessary for dependent components first, or by immediately testing a concrete implementation.
-   Feature: Implement Embedder backends e.g. OpenAI API Hugging Face.
    -   TDD successful embedding generation using settings such as API key environment variable name model ID loaded from the AppConfig struct which is populated from ~/.indexer/config.yaml.
    -   TDD error handling for API errors network issues specific to each backend.
    -   TDD rate limit handling logic and retry mechanisms if applicable and configurable via AppConfig.
-   Task: Log all embedding operations success, failure, retries.

### 4.3 Vector DB Abstraction
-   Feature: Abstract vector storage/retrieval behind a VectorStore trait.
    -   Define trait methods e.g., upsert_vectors, query_similar_vectors.
    -   TDD the trait definition.
-   Feature: Implement Redis VectorStore backend.
    -   TDD vector upsert functionality.
    -   TDD vector similarity search functionality.
    -   TDD key prefixing and entity typing enforcement.
-   Task: Add logging for all Redis/vector DB operations adhering to @logging-required-for-redis.
-   Planning Note: Plan for future Qdrant/Pinecone backends by ensuring the trait is generic enough.

### 4.4 Vector Indexing Workflow
-   Feature: Core entity extraction for vectorization reuse or adapt classic mode logic if possible, with tests ensuring no side-effects.
-   Feature: Embedding generation for extracted entities.
    -   TDD the process of taking entities, calling the Embedder, and receiving vectors.
-   Feature: Storing embeddings with metadata in the VectorStore.
    -   TDD the linkage of entities, their embeddings, and relevant metadata file, entity type, signature.
-   Feature: Batch processing for indexing.
    -   TDD batching logic.
    -   TDD progress logging during batch processing.
-   Feature: Error handling during indexing log and continue for non-fatal errors.
    -   TDD various error scenarios and confirm correct behavior.

### 4.5 Vector Recall/Search
-   Feature: Similarity search CLI command core logic.
    -   TDD retrieval of query, calling VectorStore, and getting top-K matches.
-   Feature: Output formatting for recall results.
    -   TDD human-readable output.
    -   TDD optional machine-parseable output e.g., JSON via a flag.
-   Task: Log all query parameters and results.

### 4.6 Migration & Compatibility
-   Task: Define and test strategies to ensure classic and vector data coexist without key mixing or overwrites. Test cases MUST verify this isolation, following @isolate-project-state.
-   Task: Document migration instructions if any schema changes are anticipated or become necessary.

### 4.7 Testing Strategy (Fulfilled by TDD Workflow)
-   The primary testing strategy IS the /tdd-workflow-feature-development-cycle.
-   This workflow inherently covers:
    -   Unit tests for every function and logical block of behavior.
    -   Integration tests as features combine e.g., CLI -> Embedder -> VectorStore.
    -   Testing of success and failure paths for each behavior.
    -   Ensuring no commit occurs unless all tests pass and zero warnings are present as per @zero-warnings-required.
    -   Generation of placeholder tests and halting if a behavior is to be implemented without a prior failing test specification.
-   Coverage will be tracked using tools like tarpaulin, aiming for >90 percent as a baseline, with critical paths at 100 percent.

### 4.8 Documentation (Integrated with TDD)
-   Update all READMEs and usage docs for new commands and configurations will occur as features are completed and proven through the TDD cycle, following /documentation-workflow.
-   Migration notes and troubleshooting sections will be developed based on tested scenarios and potential failure points identified during TDD.
-   Test coverage reports will form part of the documentation suite.

## 5. Forbidden Actions
- No changes to classic indexing unless required for compatibility must be documented and developed via its own TDD cycle.
- No stubs, TODOs, or incomplete code enforced by TDD workflow and @no-stubs-no-todos-no-future-work.
- No silent errors, warning suppression, or unlogged failures TDD error path testing and @error-handling-and-logging / @no-silent-errors apply.
- No creative output or deviation from this roadmap unless explicitly approved, documented, and developed via a new TDD cycle for that approved change, adhering to @no-creative-output.

## 6. Milestones
(Milestones now reflect completion of features developed via TDD)
- [ ] vectorize & vector-recall CLI command structures defined, argument parsing TDD-complete & documented.
- [ ] Embedder trait and first backend implementation TDD-complete, config loading tested.
- [ ] VectorStore trait and Redis backend TDD-complete, including key hygiene tests.
- [ ] End-to-end vector indexing workflow TDD-complete entity extraction, embedding, storing, batching, error handling.
- [ ] Vector recall/search core logic and output formatting TDD-complete.
- [ ] All integration points tested, all individual features TDD-complete, zero warnings system-wide.
- [ ] Final documentation, usage examples, and migration notes complete and verified.

---

All changes must reference this roadmap and be tracked in .windsurf/tasks.md (as per @task-tracking). Each task in .windsurf/tasks.md will be implemented following the /tdd-workflow-feature-development-cycle. No handoff or merge unless every relevant milestones underlying TDD cycles are green, refactored, and warning-free, as per @handoff-procedure.