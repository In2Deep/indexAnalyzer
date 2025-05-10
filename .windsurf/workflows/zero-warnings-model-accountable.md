---
description: Enforces zero warnings and full model accountability for code quality and test results. No excuses, no handoffs with defects.
---

# Workflow: Zero Warnings, Zero Excuses, Model-Accountable Delivery

## 1. Absolute Clean State Before Any Code Change
- Run `git status` and `git diff` to ensure no uncommitted changes or untracked files pollute the working directory.
- Stash or commit all unrelated changes before proceeding.

## 2. Compilation and Test Baseline (Pre-Change)
- Run `cargo clean && cargo build` and `cargo test --all -- --nocapture` before making any code changes.
- If any warnings or test failures are present, the model must halt and address them **before** writing new code.
- Document all pre-existing issues in `.windsurf/logs/pre-change.log`.

## 3. Code Change Responsibility
- The model, not the user, is responsible for all code changes, test results, and code hygiene.
- The model must not defer, excuse, or minimize any warning or error. Every warning is a defect.
- The model must not claim "all is passing" unless all output is green and free of warnings/errors.

## 4. Post-Change Build and Test
- After any code change, immediately run `cargo build` and `cargo test --all -- --nocapture`.
- If **any** warning or error is present (yellow or red output), the model must:
  - Halt further work.
  - Triage and fix every warning and error, one by one, until the build and test output is 100% clean (green).
  - Do not proceed with additional features, refactors, or tasks until all warnings are resolved.

## 5. No Warning Left Behind
- Warnings include, but are not limited to: unused variables, dead code, deprecated usage, redundant patterns, type inference issues, etc.
- The model must fix all warnings, not silence them with attributes or ignore them.
- If a warning cannot be fixed due to an upstream dependency, document it in `.windsurf/tasks.md` with justification and reference.

## 6. Honest and Accurate Reporting
- The model must never state "all tests pass" or "build is clean" if any warning or error is present.
- If the user is seeing red or yellow, the model must acknowledge and address it, not contradict the user's terminal or CI output.
- All status reports must be accurate, complete, and reflect the true state of the codebase as seen by the user.

## 7. Commit and Documentation
- Only commit changes when the build and test output is 100% clean (no warnings, no errors).
- Each commit message must reference the specific warning/error fixed, and include a summary of the steps taken.
- Update `.windsurf/tasks.md` and relevant logs to reflect all fixes and status.

## 8. Handoff and Review
- Before handing off to the user or another model, confirm all output is green.
- Provide a summary of what was fixed, how, and include the final clean test/build logs.
- If any warning or error remains, the handoff is not complete.

---
**Deviation from this workflow is never permitted. Warnings are defects. The model is fully responsible for code quality and must never shift blame or responsibility to the user.**
