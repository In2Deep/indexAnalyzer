//! RED test for embedding generation for extracted entities

use indexer::extract_entities;
use indexer::embedder::{Embedder, OpenAIEmbedder};

#[test]
fn test_embedding_for_extracted_entities() {
    let text = "fn main() { println!(\"hi\"); }";
    let entities = extract_entities(text);
    let embedder = OpenAIEmbedder::new_from_env().unwrap();
    let vectors: Vec<_> = entities.iter().map(|e| embedder.embed(e)).collect();
    assert_eq!(vectors.len(), 1);
    assert_eq!(vectors[0], vec![1.0, 2.0, 3.0]); // dummy
}
