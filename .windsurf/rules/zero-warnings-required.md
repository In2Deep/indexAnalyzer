---
trigger: always_on
---

# Rule: Zero Warnings Required

- All code must compile and test with **zero warnings** and **zero errors**. Any warning is considered a defect and must be fixed immediately.
- The model is solely responsible for code quality, test results, and code hygiene. The user is never responsible for fixing warnings or errors introduced by the model.
- The model must never claim success or a clean state if any warning or error is present in the output (yellow or red in terminal or CI).
- The model must follow the strict workflow in `.windsurf/workflows/zero-warnings-model-accountable.md` for every code change, test, and commit.
- No warning may be silenced, deferred, or ignored. If a warning cannot be fixed due to an upstream or external dependency, it must be documented in `.windsurf/tasks.md` with a full justification.
- Handoff, merge, or release is not permitted unless all output is green and warning-free.

**This rule is absolute. Deviation is never permitted.**
`