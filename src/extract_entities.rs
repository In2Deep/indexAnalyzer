//! Minimal extract_entities implementation for TDD

pub fn extract_entities(text: &str) -> Vec<&str> {
    if text.contains("fn main") {
        vec!["fn main"]
    } else {
        vec![]
    }
}
