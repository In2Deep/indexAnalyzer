//! RED test for Redis backend implementation (upsert/query, key prefixing, entity typing)

use indexer::vector_store::RedisVectorStore;
use std::env;
use indexer::vector_store::VectorStore;

#[test]
fn test_redis_vector_store_upsert_and_query() {
    // For the test, we'll use the trait implementation which is synchronous
    // This allows us to test without async/await
    let redis_url = env::var("REDIS_URL").unwrap_or_else(|_| "redis://localhost:6379/0".to_string());
    let key_prefix = "code:testproject";
    let store = RedisVectorStore::new(&redis_url, key_prefix);
    
    // For testing, we'll use the VectorStore trait methods which are synchronous
    let entity_id = "foo";
    let vector = vec![1.0, 2.0, 3.0];
    
    // Store the vector using the trait method
    let store_result = VectorStore::upsert_embedding(&store, entity_id, &vector, Some("test.py"), Some("doc"));
    assert!(store_result.is_ok(), "Should store vector without error");
    
    // Query using the trait method
    let results = VectorStore::similarity_search(&store, &vector, 1);
    assert!(!results.is_empty(), "Should return at least one result");
}

#[test]
fn test_redis_vector_store_key_prefixing() {
    let redis_url = env::var("REDIS_URL").unwrap_or_else(|_| "redis://localhost:6379/0".to_string());
    let key_prefix = "code:testproject";
    let store = RedisVectorStore::new(&redis_url, key_prefix);
    let full_key = store.make_key("doc", "foo");
    assert_eq!(full_key, "code:testproject:doc:foo");
}
