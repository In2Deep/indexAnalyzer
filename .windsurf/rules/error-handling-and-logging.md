---
trigger: always_on
---

All functions must handle errors gracefully and log them appropriately using the existing logging framework. No use of `unwrap()` or `expect()` outside of tests.