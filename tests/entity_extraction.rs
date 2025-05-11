//! RED test for entity extraction for vectorization

use indexer::extract_entities;


#[test]
fn test_extract_entities_basic() {
    let text = "fn main() { println!(\"hi\"); }";
    let entities = extract_entities(text);
    assert_eq!(entities, vec!["fn main"]);
}

#[test]
fn test_extract_entities_none() {
    let text = "// just a comment";
    let entities = extract_entities(text);
    assert!(entities.is_empty());
}
