---
description: Workflow
---

This workflow must be followed by all users and models working in this workspace. It is designed to ensure strict compliance with the roadmap, robust code quality, and smooth handoff between contributors and models.

---

## 1. Preparation
- Read the latest `roadmap_part*.md`, `dependency_setup.md`, and `configuration.md` in `docs/`.
- Review `RULES.md` for binding rules.
- Ensure your local environment is using only the approved, latest stable Rust toolchain and dependencies.

## 2. Code Implementation
- Before writing any Rust code, **read and understand the logic and intent of the Python `code_indexer.py`**. Do NOT translate line-by-line—fully parse and comprehend the purpose and flow.
- Implement features module-by-module as described in the roadmap, with no deviation.
- All code must be async, modular, and idiomatic.
- Use only the libraries and versions specified in `dependency_setup.md`.
- All configuration must be loaded from `~/.indexer/config.yaml`.

## 3. Testing
- Write async unit and integration tests for all new code.
- Run all tests after each major change. Code without tests is not complete.

## 4. Documentation
- Update module-level and public function doc comments as you go.
- If any new configuration keys are introduced, update `configuration.md` and the config struct.

## 5. Output Verification
- In generate mode, always verify output matches the required format byte-for-byte with the Python version.

## 6. Handoff Procedure
- Before handing off to another model or user, ensure:
  - All code is committed and pushed (if using version control).
  - All tests pass.
  - Documentation is up to date and consistent with the roadmap.
  - No stubs, TODOs, or partial implementations remain.

## 7. Review and Feedback
- All changes should be reviewed for compliance with `RULES.md` and the roadmap.
- Feedback should reference specific roadmap or rules sections.

---

**Deviations from this workflow are not allowed unless explicitly approved and documented in the roadmap.**
