---
description: automatic
---

1. For the next 10 open tasks in .windsurf/tasks.md:
   - Expand/document test cases if not explicit.
   - Write all failing test(s) (RED) for the batch, run and confirm failures.
   - Implement the minimal code to pass each test (GREEN), running tests after each change.
   - Refactor for clarity/idiomatic Rust after batch GREEN phase, re-run all tests.
   - Update all relevant documentation (README, docs, inline, etc.) for all completed tasks in a single doc update pass if possible.
   - Commit after each task or batch, with clear messages and references.
   - Mark each task as completed in .windsurf/tasks.md with summary and commit reference.
2. If tasks are independent, process test-writing and documentation updates in parallel to save time.
3. On any failure, log the error, attempt self-repair, and continue with the next task if possible.
4. After N tasks, output a summary table: Task → Status → Commit Hash → Notes.
5. All steps must strictly follow TDD, documentation, and task-tracking workflows, and comply with all project rules