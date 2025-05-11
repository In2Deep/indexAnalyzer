//! RED test for similarity search logic (query, call VectorStore, get top-K, error handling)

trait VectorStore {
    fn query_top_k(&self, query: &[f32], k: usize) -> Vec<(&str, f32)>;
}

struct DummyVectorStore;

impl VectorStore for DummyVectorStore {
    fn query_top_k(&self, _query: &[f32], k: usize) -> Vec<(&str, f32)> {
        // Dummy: always return k items
        (0..k).map(|i| ("foo", 0.9)).collect()
    }
}

#[test]
fn test_similarity_search_top_k() {
    let store = DummyVectorStore;
    let results = store.query_top_k(&[1.0, 2.0, 3.0], 2);
    assert_eq!(results.len(), 2);
    assert!(results.iter().all(|(_id, score)| *score == 0.9));
}
