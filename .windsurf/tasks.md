# Project Tasks

This file is the canonical, visible list of tracked tasks for the project. All new tasks, bugfixes, and feature requests should be added here and checked off when complete.

## Open Tasks

- [ ] Fix integration test failure for global `--name` parameter in CLI. Current implementation has `--name` defined per subcommand rather than globally as documented in the README.

- [ ] Implement `vectorize` subcommand for CLI as described in roadmap.md.
  - Add arguments: `--name`, `--model`, `--db`, `--batch-size`, `--dry-run`, `--verbose`
  - Update CLI documentation

- [ ] Implement `vector-recall` subcommand for CLI as described in roadmap.md.
  - Add arguments: `--name`, `--query`, `--top-k`, `--model`, `--db`
  - Ensure output matches recall conventions

- [ ] Create embedding trait abstraction (`Embedder`) and implement at least one backend.
  - Read configuration from `~/.indexer/config.yaml` or env vars
  - Add proper error handling and logging

- [ ] Create vector database trait abstraction (`VectorStore`) and implement Redis backend.
  - Ensure proper key prefixing for project isolation
  - Add comprehensive logging for all operations


## Completed Tasks

- Fix line number extraction in src/ast_parser.rs to use real AST node locations for all entities (function, class, assignment). Add tests and follow all workflows. (commit d4d9958)

- [x] Fix CLI to accept --name as a global argument. The current CLI only accepts positional arguments, so running the application with --name fails. Update argument parsing to support --name as documented, and ensure all commands use it for project namespacing. (Completed in commit 2bbd8a6; note: 1 integration test failure remains for follow-up). in Redis. (commit: c83692b)
