---
description: Workflow for Project Documentation and Commit Hygiene
---

# Documentation Workflow

This workflow ensures that all documentation is consistently created, updated, and maintained for every code change, feature, refactor, or handoff. It applies to all contributors and models.

---

## 1. README Maintenance
- Update the main `README.md` in the project root to reflect any new features, changes, or usage instructions.
- Update any additional README files (e.g., in `docs/`, module directories) as needed to keep them current and accurate.
- If a new module or major feature is added, create a corresponding README if one does not exist.

## 2. Code Documentation
- For every new feature, bugfix, or refactor, update or add doc comments for all public functions, structs, and modules.
- If code behavior or configuration changes, update related documentation in `docs/` (including `configuration.md`, `roadmap_part*.md`, etc.).
- For refactors, briefly document what was changed and why in the relevant README or module-level comment.

## 3. Commit Documentation
- Every commit must include a clear, descriptive message summarizing what was changed and why.
- If the commit addresses a tracked task, reference the task in the commit message (e.g., `Fixes #task-id`).
- For multi-step changes, summarize the overall goal and each major change.

## 4. Review Before Commit
- Before committing, review all README files and docs to ensure they accurately reflect the current state of the codebase.
- Ensure all new or changed code is properly documented.
- If documentation is missing or incomplete, update it before proceeding.

## 5. Handoff and Task Tracking
- When handing off work or closing a task, confirm that all relevant documentation is up to date.
- Move completed documentation tasks to the `Completed Tasks` section in `.windsurf/tasks.md` with a brief note and commit reference.

---

**Deviation from this workflow is not permitted unless explicitly approved and documented in the roadmap.**
