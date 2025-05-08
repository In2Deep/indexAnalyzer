# STRICT ROADMAP (CONTINUED)

## 8. AST Parsing & Code Entity Extraction (ast_parser.rs)

8.1. Use `rustpython-ast` for parsing. **DO NOT** use any other parser.
8.2. Define `CodeEntity` struct exactly as below (fields/serde attributes must match):
```rust
use serde::Serialize;

#[derive(Serialize, Debug, Clone)]
pub struct CodeEntity {
    pub entity_type: String, // "function", "class", "method", "variable"
    pub file_path: String,   // Relative path from app_dir (POSIX)
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signature: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub docstring: Option<String>,
    pub line_start: usize,
    pub line_end: usize,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_class: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bases: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value_repr: Option<String>,
}
```
8.3. Implement `extract_entities_from_file(file_path: &PathBuf, app_dir: &PathBuf) -> Result<Vec<CodeEntity>, AppError>`:
- Read file async with `tokio::fs::read_to_string`.
- Parse with `rustpython_parser::parser::parse_program`.
- On error, log and return `AppError::AstParse`.
- Traverse AST recursively, passing parent context as needed (do NOT assign parent attributes).
- Extract all required fields for each entity, as described in `promopt.md`.
- Signature construction: **DO NOT** use line-matching heuristics. Instead, build from AST structure as described.
- Continue on recoverable node errors; skip file on fatal parse error.

---

## 9. Redis Operations (redis_ops.rs)

9.1. Use `fred` crate for all Redis communication.
- Only in `direct` mode: connect, wait, ping.
- Keys and commands must mirror Python logic:
  - If `entity_type == "method"`, key is `{key_prefix}:method:{file_path}:{parent_class}.{name}`.
  - Else, `{key_prefix}:{entity_type}:{file_path}:{name}`.
- Implement `process_set_command` as described in `promopt.md`.
- **DO NOT** use blocking or sync Redis calls.

---

## 10. Output & Testing

10.1. In generate mode, output MCP commands to stdout. **MUST** match Python output byte-for-byte.
10.2. Implement comprehensive async tests for all modules. **DO NOT** skip error paths.

---

## 11. Forbidden Actions

- **DO NOT** introduce any new libraries, patterns, or architectural changes.
- **DO NOT** add extra features, refactorings, or "improvements" unless explicitly instructed.
- **DO NOT** skip or alter any error handling, logging, or output requirements.

---

**END OF STRICT ROADMAP**

If the content above does not fit in a single file, continue splitting as needed and reference previous parts.
