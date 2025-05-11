//! RED test for Redis backend implementation (upsert/query, key prefixing, entity typing)

use indexer::vector_store::RedisVectorStore;
use std::env;

#[test]
fn test_redis_vector_store_upsert_and_query() {
    // Use a test Redis URL and key prefix
    let redis_url = env::var("REDIS_URL").unwrap_or_else(|_| "redis://localhost:6379/0".to_string());
    let key_prefix = "code:testproject";
    let mut store = RedisVectorStore::new(&redis_url, key_prefix);
    let entity_type = "doc";
    let key = "foo";
    let vector = vec![1.0, 2.0, 3.0];
    store.upsert(entity_type, key, &vector).unwrap();
    let result = store.query(entity_type, key).unwrap();
    assert_eq!(result, vector);
}

#[test]
fn test_redis_vector_store_key_prefixing() {
    let redis_url = env::var("REDIS_URL").unwrap_or_else(|_| "redis://localhost:6379/0".to_string());
    let key_prefix = "code:testproject";
    let store = RedisVectorStore::new(&redis_url, key_prefix);
    let full_key = store.make_key("doc", "foo");
    assert_eq!(full_key, "code:testproject:doc:foo");
}
