//! RED test for VectorStore trait abstraction and mock/test impl

trait VectorStore {
    fn upsert(&mut self, key: &str, vector: Vec<f32>);
    fn query(&self, key: &str) -> Option<&Vec<f32>>;
}

struct MockVectorStore {
    store: std::collections::HashMap<String, Vec<f32>>,
}

impl MockVectorStore {
    fn new() -> Self {
        Self { store: std::collections::HashMap::new() }
    }
}

impl VectorStore for MockVectorStore {
    fn upsert(&mut self, key: &str, vector: Vec<f32>) {
        self.store.insert(key.to_string(), vector);
    }
    fn query(&self, key: &str) -> Option<&Vec<f32>> {
        self.store.get(key)
    }
}

#[test]
fn test_vector_store_upsert_and_query() {
    let mut store = MockVectorStore::new();
    store.upsert("foo", vec![1.0, 2.0, 3.0]);
    let result = store.query("foo");
    assert_eq!(result, Some(&vec![1.0, 2.0, 3.0]));
}

#[test]
fn test_vector_store_query_missing() {
    let store = MockVectorStore::new();
    assert!(store.query("missing").is_none());
}
