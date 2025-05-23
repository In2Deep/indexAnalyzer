---
description: Critical fix for never-type fallback warnings in redis_ops.rs
---

This workflow step must be followed with zero deviation. Warnings related to never type fallback in Rust are not cosmetic — they will become compile-time errors in Rust 2024. These must be addressed immediately and permanently.

Objective

Fix all never type fallback warnings in redis_ops.rs by explicitly specifying return types or discarding unused values in a Rust-idiomatic and safe manner. Do not suppress the warnings. Do not silence them with let _ = if the fallback type is still being inferred. Your job is to eliminate the root cause.

Step-by-step

1. Identify exact warnings  
Run:  
cargo clean && cargo test --all --nocapture | tee bugfix-log.txt  
Then open bugfix-log.txt and find the lines referencing:

- store_file_content
- store_code_entities
- clear_file_data

These are where fallback is happening.

2. Analyze each case  
In each affected function:
- Do not guess types or use placeholder fixes.
- Use let _: Result<(), Error> if you must discard values but only after you've proven the call always returns Result.
- Otherwise, explicitly annotate the return type, or unwrap safely (e.g., using .map(|_| ()), etc.)

You are NOT allowed to patch over this by silencing the warning. You must fix the underlying type inference problem.

3. Test and verify  
After each individual fix:  
cargo test redis_ops --nocapture  
Confirm the warning is completely gone  
If it's still there, your fix is wrong. Try again. No hand-waving.

4. Commit  
After confirming the fix removes the warning, do:  
git add src/redis_ops.rs  
git commit -m "fix: remove never-type fallback warning in store_file_content"  
Repeat for each function. One commit per fix. Do not batch these.

5. Final sweep  
Run a full test and confirm zero fallback warnings:  
cargo clean && cargo test --all --nocapture  
If any are left: go back and fix them. If none remain: move to the next workflow item.

Absolutely forbidden
- #[allow(warning)] or any compiler attribute meant to silence the warning
- Comments downplaying the impact
- Using let _ = just to shut the compiler up
- Committing changes without confirming the warning is resolved

This is not a discussion. This is not a debate. This is not "just a warning." This is a countdown to failure in Rust 2024. Fix it or GTFO.