---
trigger always_on
---

- Any edit inside a function body signature docstring or even reordering must trigger full test execution for that functions module
- No commits allowed unless all related tests pass
- If no test exists halt execution and generate a skeleton test case before continuing
- Models may not bypass this with justification or small change excuses No free passes