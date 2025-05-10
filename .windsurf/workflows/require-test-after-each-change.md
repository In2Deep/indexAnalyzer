---
description: 
---


  "title": "nuclear-test-discipline",
  "description": "Forces test validation for any code change within function scope. No change is too small to escape scrutiny.",
  "body": [
    "## 1. Trigger Conditions",
    "- Any edit inside a function—including renames, reorderings, or doc changes—triggers this workflow.",
    "- Applies to user or model edits. No bypass allowed.",
    "",
    "## 2. Immediate Actions",
    "- Identify the edited function and its containing module.",
    "- Check for associated tests. If none exist, generate a placeholder test and halt execution.",
    "- Run all tests for that module immediately after the change.",
    "",
    "## 3. Enforcement",
    "- Do not proceed if any related test fails.",
    "- Do not accept 'trivial change' as a justification to skip testing.",
    "- Model must confirm test status in its output before continuing.",
    "",
    "## 4. Logging",
    "- Log every test run triggered by this workflow for audit.",
    "- Include filename, function name, and test results.",
    "",
    "## 5. Violation Handling",
    "- If code was modified and this workflow was not triggered, flag the session for manual review.",
    "- Repeat offenders must be shamed with sarcastic commit messages.",
    "## 6. Documentation",
    "- Documentation is very, very poor and must be improved. Focus all docs on the Rust implementation; do not add or update any documentation for the Python code.",
    "- After every successful test or bug fix, update or create the corresponding documentation entry for the affected module/function."
 