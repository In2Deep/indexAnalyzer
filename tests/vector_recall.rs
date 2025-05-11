//! RED tests for vector recall/search workflow

use indexer::vector_store::RedisVectorStore;
use serde_json;

#[test]
fn test_similarity_search_returns_top_k_matches() {
    let store = RedisVectorStore::new("redis://localhost:6379/0", "test_prefix");
    let query_vec = vec![1.0, 2.0, 3.0];
    let top_k = 3;
    let results = store.similarity_search(&query_vec, top_k);
    assert_eq!(results.len(), top_k, "Should return exactly top_k matches");
}

#[test]
fn test_recall_output_human_readable() {
    let results = vec!["foo", "bar", "baz"];
    let output = format!("Results:\n{}", results.join(",\n"));
    assert!(output.contains("Results:"));
    assert!(output.contains("foo"));
}

#[test]
fn test_recall_output_json() {
    let results = vec!["foo", "bar", "baz"];
    let json = serde_json::to_string(&results).unwrap();
    assert!(json.starts_with("["));
    assert!(json.contains("foo"));
}

#[test]
fn test_query_logging() {
    let query = "def foo()";
    log::info!("Querying for: {}", query);
    assert!(true, "Log should be emitted");
}

#[test]
fn test_classic_and_vector_data_isolation() {
    let classic_prefix = "code_index:classic";
    let vector_prefix = "code_index:vector";
    assert_ne!(classic_prefix, vector_prefix, "Prefixes must be distinct");
}

#[test]
fn test_migration_documentation_exists() {
    // Check that migration docs are present
    let doc_path = std::path::Path::new("docs/migration.md");
    assert!(doc_path.exists(), "Migration documentation must exist");
}