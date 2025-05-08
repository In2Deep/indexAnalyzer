//! ast parsing for code_indexer_rust
//! - parses python source files and extracts entities

use rustpython_ast::{ast, parser};
use std::fs;
use std::path::Path;

#[derive(Debug, Clone)]
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
    let source_lines: Vec<&str> = content.lines().collect();
    let rel_path = file_path.strip_prefix(base_dir).unwrap_or(file_path).to_string_lossy().to_string();
    let mut entities = Vec::new();
    let ast = match parser::parse_program(&content, "<embedded>") {
        Ok(a) => a,
        Err(_) => return vec![],
    };
    use rustpython_ast::ast::*;
    use rustpython_ast::walk_ast;
    fn get_line_range(node: &impl std::fmt::Debug) -> (usize, usize) {
        // fallback: line 1-1
        (1, 1)
    }
    fn get_docstring(body: &[Box<Stmt>]) -> Option<String> {
        if let Some(Stmt::Expr { value, .. }) = body.get(0).map(|b| &**b) {
            if let Expr::Constant { value: Constant::Str(s), .. } = &**value {
                return Some(s.clone());
            }
        }
        None
    }
    fn get_signature(name: &str, args: &Arguments) -> String {
        let mut sig = format!("def {}(", name);
        let mut parts = Vec::new();
        for arg in &args.args {
            parts.push(arg.arg.clone());
        }
        sig.push_str(&parts.join(", "));
        sig.push(')');
        sig
    }
    fn walk(node: &Stmt, rel_path: &str, entities: &mut Vec<CodeEntity>, parent_class: Option<&str>) {
        use rustpython_ast::ast::*;
        match node {
            Stmt::FunctionDef { name, args, body, .. } => {
                let (line_start, line_end) = (1, 1); // TODO: get real lines
                let docstring = get_docstring(body);
                entities.push(CodeEntity {
                    entity_type: if parent_class.is_some() { "method" } else { "function" }.to_string(),
                    file_path: rel_path.to_string(),
                    name: name.clone(),
                    signature: Some(get_signature(name, args)),
                    docstring,
                    line_start,
                    line_end,
                    parent_class: parent_class.map(|s| s.to_string()),
                    bases: None,
                    value_repr: None,
                });
            }
            Stmt::ClassDef { name, bases, body, .. } => {
                let (line_start, line_end) = (1, 1); // TODO: get real lines
                let docstring = get_docstring(body);
                let base_names = bases.iter().map(|b| format!("{:?}", b)).collect();
                entities.push(CodeEntity {
                    entity_type: "class".to_string(),
                    file_path: rel_path.to_string(),
                    name: name.clone(),
                    signature: None,
                    docstring,
                    line_start,
                    line_end,
                    parent_class: None,
                    bases: Some(base_names),
                    value_repr: None,
                });
                for stmt in body {
                    walk(stmt, rel_path, entities, Some(name));
                }
            }
            Stmt::Assign { targets, value, .. } => {
                // Only top-level or class-level
                for target in targets {
                    if let Expr::Name { id, .. } = &**target {
                        entities.push(CodeEntity {
                            entity_type: "variable".to_string(),
                            file_path: rel_path.to_string(),
                            name: id.clone(),
                            signature: None,
                            docstring: None,
                            line_start: 1,
                            line_end: 1,
                            parent_class: parent_class.map(|s| s.to_string()),
                            bases: None,
                            value_repr: Some(format!("{:?}", value)),
                        });
                    }
                }
            }
            _ => {}
        }
        // Recurse into children
        if let Stmt::ClassDef { body, .. } = node {
            for stmt in body {
                walk(stmt, rel_path, entities, parent_class);
            }
        }
    }
    for stmt in &ast {
        walk(stmt, &rel_path, &mut entities, None);
    }
    entities
}
