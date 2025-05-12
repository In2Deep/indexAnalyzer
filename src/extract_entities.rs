//! Entity extraction for vectorization
//! - Extracts code entities from text for embedding and vector search
//! - Uses rustpython-parser to parse Python code
//! - Also handles basic Rust code patterns
//! - Returns a list of entity strings suitable for embedding

use rustpython_ast::*;
use rustpython_parser::ast::Suite;
use rustpython_parser::Parse;
use log::warn;

/// Extract code entities from text for vectorization
/// 
/// This function parses code and extracts function names, class names,
/// and other entities that can be used for embedding and vector search.
/// It handles both Python and basic Rust code patterns.
/// 
/// # Arguments
/// * `text` - The code text to extract entities from
/// 
/// # Returns
/// A vector of entity strings suitable for embedding
pub fn extract_entities(text: &str) -> Vec<String> {
    // Check if it's Rust code by looking for Rust-specific syntax
    if text.contains("fn ") && (text.contains("{") || text.contains(";")) {
        return extract_entities_from_rust(text);
    }
    
    // Otherwise, try to parse as Python
    let ast = match Suite::parse(text, "<embedded>") {
        Ok(ast) => ast,
        Err(e) => {
            warn!("Failed to parse Python code: {}", e);
            return vec![];
        }
    };
    
    let mut entities = Vec::new();
    for stmt in &ast {
        extract_entities_from_stmt(stmt, &mut entities);
    }
    entities
}

/// Extract entities from Rust code using simple pattern matching
fn extract_entities_from_rust(text: &str) -> Vec<String> {
    let mut entities = Vec::new();
    
    // Simple pattern matching for Rust functions
    if let Some(_) = text.find("fn main") {
        entities.push("fn main".to_string());
    }
    
    // Add more Rust patterns as needed
    
    entities
}

/// Extract entities from a statement recursively
fn extract_entities_from_stmt(stmt: &Stmt, entities: &mut Vec<String>) {
    match stmt {
        Stmt::FunctionDef(def) => {
            // Extract function name
            entities.push(format!("fn {}", def.name));
            
            // Extract docstring if present
            if let Some(docstring) = get_docstring(&def.body) {
                entities.push(format!("doc {}: {}", def.name, docstring));
            }
            
            // Recursively extract from function body
            for stmt in &def.body {
                extract_entities_from_stmt(stmt, entities);
            }
        },
        Stmt::ClassDef(def) => {
            // Extract class name
            entities.push(format!("class {}", def.name));
            
            // Extract docstring if present
            if let Some(docstring) = get_docstring(&def.body) {
                entities.push(format!("doc {}: {}", def.name, docstring));
            }
            
            // Recursively extract from class body
            for stmt in &def.body {
                extract_entities_from_stmt(stmt, entities);
            }
        },
        Stmt::Assign(assign) => {
            // Extract variable assignments at module level
            for target in &assign.targets {
                if let Expr::Name(boxed_id) = target {
                    entities.push(format!("var {}", boxed_id.id));
                }
            }
        },
        _ => {}
    }
}

/// Extract docstring from a list of statements
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

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_extract_entities_function() {
        let text = "def foo():\n    pass\n";
        let entities = extract_entities(text);
        assert!(entities.contains(&"fn foo".to_string()));
    }
    
    #[test]
    fn test_extract_entities_class() {
        let text = "class Bar:\n    pass\n";
        let entities = extract_entities(text);
        assert!(entities.contains(&"class Bar".to_string()));
    }
    
    #[test]
    fn test_extract_entities_with_docstring() {
        let text = "def foo():\n    \"\"\"This is a docstring\"\"\"\n    pass\n";
        let entities = extract_entities(text);
        assert!(entities.contains(&"fn foo".to_string()));
        assert!(entities.contains(&"doc foo: This is a docstring".to_string()));
    }
    
    #[test]
    fn test_extract_entities_empty() {
        let text = "# just a comment";
        let entities = extract_entities(text);
        assert!(entities.is_empty());
    }
}
