---
description: Bug Fixing
---



1 clean slate  
- run git status and confirm no uncommitted changes  
- run git diff and either commit or stash any work in progress  

2 reproduce  
- execute cargo clean && cargo test --all --nocapture  
- save full console output to bugfix-logtxt  

3 triage  
- read bugfix-logtxt and list each warning or failure in order giving each a brief title  
  examples  
    1 Deprecated parse_program usage  
    2 Unused function get_line_range  
    3 Never-type fallback in redis_ops  
    4 Unused imports and variables  
    5 Dead variants in AppError  

4 fix one  
- select the first issue from your list  
- apply the minimal code change to eliminate that one warning or error  
  • replace deprecated functions with their modern equivalents  
  • remove or underscore unused code  
  • specify concrete types to avoid never-type fallbacks  
  • remove unused enum variants  
- run cargo test <module-name> --nocapture to verify the module tests pass and no new warnings appear  

5 verify all  
- run cargo test --all --nocapture  
- confirm zero warnings and zero failures project-wide  

6 commit  
- stage only the files you changed  
- commit with a precise message referencing the issue title eg  
  git commit -m fix replace deprecated parse_program with Suiteparse  
- ensure one commit per issue  

7 next item  
- remove the resolved issue from your list  
- repeat steps 4–6 for the next issue until all are fixed  

8 complete and handoff  
- perform a final cargo clean && cargo test --all  
- push your commits and open a pull request with a checklist of fixed items  
- do not merge until CI passes and a reviewer confirms no free-form or extra edits were introduced