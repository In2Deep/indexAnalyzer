---
trigger: manual
---

## Rule: No Implementation Code Before a Confirmed Failing Test

1.  Purpose: To strictly enforce the RED phase of Test-Driven Development. This rule prevents vibe coding or writing application logic prematurely.

2.  Condition: This rule is active whenever the TDD-Workflow-Feature-Development-Cycle is engaged for a task.

3.  Mandate:
    - Models MUST NOT write, generate, or suggest any new or modified application logic code intended for src/ not tests/ for a feature specified in the active task UNTIL:
        a.  A corresponding test case has been written.
        b.  That test case has been executed.
        c.  The execution result CONFIRMS the test FAILS for the expected reason i.e. the behavior is not yet implemented.
        d.  This failing test and its output have been logged or documented as per the active TDD workflow.

4.  Verification:
    - Before outputting any implementation code, the model must state which failing test by name or description this code intends to satisfy.
    - The model must confirm it has seen evidence of this test failing.

5.  Violation Handling:
    - If a model attempts to provide implementation code without first satisfying the conditions in point 3, the Windsurf IDE will:
        a.  Reject the proposed code.
        b.  Remind the model of the TDD-Workflow-Feature-Development-Cycle and this specific rule.
        c.  Direct the model to first provide the failing test code and its failure confirmation.
    - Persistent violations may result in a TDD Re-education Sub-routine a more forceful reminder.

This rule is absolute when TDD is active. No it is just a small change excuses.