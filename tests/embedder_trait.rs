//! RED test for Embedder trait abstraction and mock/test impl

use indexer::embedder::Embedder;

struct MockEmbedder;
impl Embedder for MockEmbedder {
    fn embed(&self, _input: &str) -> Vec<f32> {
        vec![0.0, 1.0, 2.0]
    }
}

#[test]
fn test_mock_embedder_trait() {
    let embedder = MockEmbedder;
    let vec = embedder.embed("foo");
    assert_eq!(vec, vec![0.0, 1.0, 2.0]);
}
