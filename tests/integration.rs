//! async integration tests for code_indexer_rust

#[tokio::test]
async fn test_config_load_default() {
    use code_indexer_rust::config::AppConfig;
    let cfg = AppConfig::load().unwrap();
    assert!(cfg.redis_url.is_some());
    assert!(cfg.log_level.is_some());
}

use code_indexer_rust::redis_ops::{create_redis_client, store_file_content, store_code_entities, query_code_entity, clear_file_data};
use fred::interfaces::SetsInterface;
use code_indexer_rust::ast_parser::CodeEntity;
use tempfile::tempdir;
use std::path::PathBuf;

#[tokio::test]
async fn test_store_and_query_file_content() {
    let redis = create_redis_client("redis://localhost:6379/15").await.unwrap();
    let key_prefix = "test";
    let rel_path = "foo/bar.py";
    let content = "def foo(): pass";
    let size = content.len();
    let mtime = 0;
    store_file_content(&redis, key_prefix, rel_path, content, size, mtime).await.unwrap();
    // Query file_index
    let files: Vec<String> = redis.smembers(format!("{}:file_index", key_prefix)).await.unwrap();
    assert!(files.contains(&rel_path.to_string()));
    // Cleanup
    clear_file_data(&redis, key_prefix, &[rel_path.to_string()]).await.unwrap();
}

#[tokio::test]
async fn test_store_and_query_entities() {
    let redis = create_redis_client("redis://localhost:6379/15").await.unwrap();
    let key_prefix = "test";
    let entity = CodeEntity {
        entity_type: "function".to_string(),
        file_path: "foo/bar.py".to_string(),
        name: "foo".to_string(),
        signature: Some("def foo()".to_string()),
        docstring: Some("doc".to_string()),
        line_start: 1,
        line_end: 2,
        parent_class: None,
        bases: None,
        value_repr: None,
    };
    store_code_entities(&redis, key_prefix, &[entity.clone()]).await.unwrap();
    let result = query_code_entity(&redis, key_prefix, "function", Some("foo")).await.unwrap();
    assert!(result.iter().any(|e| e.name == "foo"));
    // Cleanup
    clear_file_data(&redis, key_prefix, &["foo/bar.py".to_string()]).await.unwrap();
}
