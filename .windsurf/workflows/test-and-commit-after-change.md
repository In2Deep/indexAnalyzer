---
description: Forces test validation for any code change within function scope. No change is too small to escape scrutiny.
---


Automates testing and committing every code change, even tiny fixes. Writes minimal tests, ensures they pass, and commits with git. Fast, diligent, avoids dumb errors. (134 chars)

Trigger
  Runs automatically after any code change (even a one liner) upon saving in Windsurf.

Tasks
  Write a Minimal Test
  Write a quick, focused test for the change (e.g., test a tweaked function’s output).
  Keep it minimal—no chaos tests, just enough to verify the fix.

Run the Test
  Execute the test in Windsurf’s terminal (e.g., cargo test <module> nocapture).
  If it fails, fix the code or test until it passes without breaking quality.

Commit with Git
  Commit the change and test: git commit m fix: <describe change, e.g., quack overflow>.
  Keep the message clear and specific.

Verify the Commit
  Check the commit message isn’t garbage.
  Confirm only the changed file and test are committed.

Rules
  Redacted: Specify rules in the prompt.

Automation
  Hooks into Windsurf’s save event (e.g., VS Code task or extension).

Completion
  Log test results, commit hash, and applied rules to .windsurf/logs/testandcommit.log.
  Confirm workflow ran autonomously.
  Reset for next change: monitor for new saves to loop back to Tasks.