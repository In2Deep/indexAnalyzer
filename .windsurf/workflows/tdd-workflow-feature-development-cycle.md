---
description: 
---

## Objective: Implement features or changes using a strict Red-Green-Refactor TDD cycle.
Models WILL NOT write implementation code before defining and running a failing test.
This workflow ensures compliance with TDD principles and integrates with existing rules like zero-warnings-required and task-tracking.

## Phase 1: Task Definition & Test Specification (THE PLAN)
1.  Identify Task: Ensure the current work corresponds to an open item in .windsurf/tasks.md. If not, create one. The task MUST clearly define the feature or behavior to be implemented or changed.
2.  Specify Test Case(s): Before writing ANY test code, describe the test cases in plain language within the task item in .windsurf/tasks.md. Include:
    - What function or module is being tested.
    - Inputs to be provided.
    - Expected outputs or behavior.
    - How success or failure will be determined.
3.  No Implementation Code: At this stage, models are FORBIDDEN from writing or modifying any application logic related to the feature.

## Phase 2: Write Failing Test (RED)
1.  Implement Test Code: Write the actual test code based on the specification from Phase 1. This test MUST target behavior not yet implemented or existing behavior that needs to change.
    - Adhere to mandatory-testing every function gets a test.
    - For async code, use the attribute tokio double_colon test.
    - For dependencies e.g. Redis, use appropriate test doubles or stubs for unit tests or prepare for integration testing as per isolate-project-state and general advice on minimizing mocks.
2.  Run Test & Confirm Failure: Execute the test e.g. cargo test your_new_test_name --nocapture.
    - The test MUST FAIL. This is critical. It proves the test is valid and the behavior is not already present.
    - Log the failure output as per logging-required-for-redis if applicable, or general test logs.
3.  Commit Failing Test Optional but Recommended:
    - git add path/to/test_file.rs
    - git commit -m feat(TDD): Add failing test for specific_behavior_from_task

## Phase 3: Write Minimal Code to Pass (GREEN)
1.  Implement Application Code: Write the absolute MINIMUM amount of application code required to make the failing test from Phase 2 pass.
    - Adhere to no-creative-output and no-free-thinking. Stick to the requirements.
    - Ensure error-handling-and-logging and no-silent-errors are followed.
2.  Run Tests: Execute all relevant tests e.g. cargo test --nocapture.
    - The specific test from Phase 2 MUST now PASS.
    - All other existing tests in the module or project MUST continue to PASS.
    - There must be ZERO warnings, as per zero-warnings-required.
3.  Iterate if Necessary: If tests do not pass, debug and modify ONLY the implementation code. Do not change the test unless it was flawed which should have been caught in Phase 2.

## Phase 4: Refactor (CLEANUP)
1.  Refactor Code: With all tests passing, refactor BOTH the implementation code and the test code for clarity, performance, and maintainability.
    - Ensure idiomatic Rust async-modular-idiomatic-rust.
    - Remove duplication.
    - Improve names, structure, etc.
2.  Run Tests Continuously: After each refactoring change, re-run all tests to ensure they remain GREEN and warning-free.
    - If any test fails, the refactoring introduced a bug. Fix it immediately.
3.  Commit Green & Refactored Code:
    - git add .
    - git commit -m feat: Implement specific_behavior_from_task and pass tests or fix:, refactor:, etc. as appropriate

## Phase 5: Cycle Completion & Handoff
1.  Verify All Rules: Confirm compliance with handoff-procedure, model-user-handoff, no-stubs-no-todos-no-future-work.
2.  Update Task Tracker: Mark the item in .windsurf/tasks.md as completed, referencing the commits.
3.  Proceed to the next task or request further instructions.

## Strictly Forbidden in this Workflow:
- Writing implementation code before a documented, failing test has been run and confirmed.
- Modifying test code to make it pass without changing implementation code unless the test itself was fundamentally flawed.
- Committing code that does not pass all tests or has warnings.
- Ignoring any step of this Red-Green-Refactor cycle.