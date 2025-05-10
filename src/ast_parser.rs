//! ast parsing for indexer
//! - parses python source files and extracts entities

use rustpython_ast::*;
use rustpython_parser::ast::Suite;
use rustpython_parser::Parse;
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CodeEntity {
    pub entity_type: String,
    pub file_path: String,
    pub name: String,
    pub signature: Option<String>,
    pub docstring: Option<String>,
    pub line_start: usize,
    pub line_end: usize,
    pub parent_class: Option<String>,
    pub bases: Option<Vec<String>>,
    pub value_repr: Option<String>,
}

pub fn extract_code_info(file_path: &Path, base_dir: &Path) -> Vec<CodeEntity> {
    let content = match fs::read_to_string(file_path) {
        Ok(c) => c,
        Err(_) => return vec![],
    };

    let rel_path = file_path.strip_prefix(base_dir).unwrap_or(file_path).to_string_lossy().to_string();
    let mut entities = Vec::new();
    let ast = match Suite::parse(&content, "<embedded>") {
        Ok(a) => a,
        Err(_) => return vec![],
    };


    fn get_docstring(body: &[Stmt]) -> Option<String> {
        if let Some(Stmt::Expr(expr)) = body.first() {
            if let Expr::Constant(boxed_const) = &*expr.value {
                if let Constant::Str(val) = &boxed_const.value {
                    return Some(val.clone());
                }
            }
        }
        None
    }

    fn get_signature(name: &str, args: &Arguments) -> String {
        let mut sig = format!("def {}(", name);
        let mut parts = Vec::new();
        for arg in &args.posonlyargs {
            parts.push(arg.def.arg.to_string());
        }
        for arg in &args.args {
            parts.push(arg.def.arg.to_string());
        }
        for arg in &args.kwonlyargs {
            parts.push(arg.def.arg.to_string());
        }
        if let Some(arg) = &args.vararg {
            parts.push(format!("*{}", arg.arg));
        }
        if let Some(arg) = &args.kwarg {
            parts.push(format!("**{}", arg.arg));
        }
        sig.push_str(&parts.join(", "));
        sig.push(')');
        sig
    }

    fn textsize_to_line(src: &str, pos: rustpython_parser::ast::TextSize) -> usize {
    // TextSize is a byte offset; count newlines up to that offset
    let idx = pos.to_usize();
    src[..idx].lines().count() + 1 // 1-based line number
}

fn walk(node: &Stmt, rel_path: &str, entities: &mut Vec<CodeEntity>, parent_class: Option<&str>, src: &str) {
        match node {
            Stmt::FunctionDef(def) => {
                let line_start = textsize_to_line(src, def.range.start());
                let line_end = textsize_to_line(src, def.range.end());
                let docstring = get_docstring(&def.body);
                entities.push(CodeEntity {
                    entity_type: if parent_class.is_some() { "method" } else { "function" }.to_string(),
                    file_path: rel_path.to_string(),
                    name: def.name.to_string(),
                    signature: Some(get_signature(&def.name, &def.args)),
                    docstring,
                    line_start,
                    line_end,
                    parent_class: parent_class.map(|s| s.to_string()),
                    bases: None,
                    value_repr: None,
                });
            }
            Stmt::ClassDef(def) => {
                let line_start = textsize_to_line(src, def.range.start());
                let line_end = textsize_to_line(src, def.range.end());
                let docstring = get_docstring(&def.body);
                let base_names = def.bases.iter().map(|b| format!("{:?}", b)).collect();
                entities.push(CodeEntity {
                    entity_type: "class".to_string(),
                    file_path: rel_path.to_string(),
                    name: def.name.to_string(),
                    signature: None,
                    docstring,
                    line_start,
                    line_end,
                    parent_class: None,
                    bases: Some(base_names),
                    value_repr: None,
                });
                for stmt in &def.body {
                    walk(stmt, rel_path, entities, Some(&def.name), src);
                }
            }
            Stmt::Assign(assign) => {
                // Only top-level or class-level
                for target in &assign.targets {
                    if let Expr::Name(boxed_id) = target {
                        entities.push(CodeEntity {
                            entity_type: "variable".to_string(),
                            file_path: rel_path.to_string(),
                            name: boxed_id.id.to_string(),
                            signature: None,
                            docstring: None,
                            line_start: 1,
                            line_end: 1,
                            parent_class: parent_class.map(|s| s.to_string()),
                            bases: None,
                            value_repr: Some(format!("{:?}", assign.value)),
                        });
                    }
                }
            }
            _ => {}
        }
        // Recurse into children
        if let Stmt::ClassDef(def) = node {
            for stmt in &def.body {
                walk(stmt, rel_path, entities, parent_class, src);
            }
        }
    }
    for stmt in &ast {
        walk(stmt, &rel_path, &mut entities, None, &content);
    }
    entities
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;
    use tempfile::tempdir;
    #[test]
    fn test_extract_code_info_lines() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("foo.py");
        let code = "class Bar:\n    def foo(self):\n        pass\n";
        let mut file = File::create(&file_path).unwrap();
        write!(file, "{}", code).unwrap();
        let entities = extract_code_info(&file_path, dir.path());
        assert!(entities.iter().any(|e| e.name == "Bar" && e.line_start > 0));
        assert!(entities.iter().any(|e| e.name == "foo" && e.line_start > 0));
    }
}

