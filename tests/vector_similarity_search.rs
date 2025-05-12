//! RED tests for vector similarity search with more comprehensive requirements
//! Tests similarity search, scoring, filtering, and result formatting

use indexer::vector_store::RedisVectorStore;
use indexer::vector_search::{search_vectors, SearchResult, SearchOptions};
use indexer::output_format::{format_search_results, OutputFormat};
use std::collections::HashMap;

#[test]
fn test_similarity_search_with_scoring() {
    // This test verifies that search results include similarity scores
    let store = RedisVectorStore::new("redis://localhost:6379/0", "test_prefix");
    let query_vec = vec![1.0, 2.0, 3.0];
    let top_k = 3;
    
    // Call the search_vectors function which should return results with scores
    let options = SearchOptions {
        top_k,
        min_score: Some(0.5),
        entity_types: None,
        file_filter: None,
    };
    
    let results = search_vectors(&store, &query_vec, &options);
    
    // Verify results
    assert!(results.is_ok(), "Search should succeed");
    let search_results = results.unwrap();
    
    assert_eq!(search_results.len(), top_k, "Should return exactly top_k matches");
    
    // Verify each result has a score
    for result in &search_results {
        assert!(result.score > 0.0, "Each result should have a positive score");
        assert!(result.score <= 1.0, "Scores should be normalized between 0 and 1");
        assert!(!result.entity_id.is_empty(), "Entity ID should not be empty");
        assert!(result.metadata.contains_key("type"), "Metadata should include entity type");
    }
    
    // Verify results are sorted by score in descending order
    for i in 1..search_results.len() {
        assert!(search_results[i-1].score >= search_results[i].score, 
                "Results should be sorted by score in descending order");
    }
}

#[test]
fn test_similarity_search_with_filtering() {
    // This test verifies that search results can be filtered by entity type and file
    let store = RedisVectorStore::new("redis://localhost:6379/0", "test_prefix");
    let query_vec = vec![1.0, 2.0, 3.0];
    
    // Test filtering by entity type
    let type_options = SearchOptions {
        top_k: 5,
        min_score: None,
        entity_types: Some(vec!["function".to_string()]),
        file_filter: None,
    };
    
    let type_results = search_vectors(&store, &query_vec, &type_options);
    assert!(type_results.is_ok(), "Search with type filter should succeed");
    let type_search_results = type_results.unwrap();
    
    // Verify all results are of the specified type
    for result in &type_search_results {
        assert_eq!(result.metadata.get("type").unwrap(), "function", 
                  "All results should be of type 'function'");
    }
    
    // Test filtering by file
    let file_options = SearchOptions {
        top_k: 5,
        min_score: None,
        entity_types: None,
        file_filter: Some("test.py".to_string()),
    };
    
    let file_results = search_vectors(&store, &query_vec, &file_options);
    assert!(file_results.is_ok(), "Search with file filter should succeed");
    let file_search_results = file_results.unwrap();
    
    // Verify all results are from the specified file
    for result in &file_search_results {
        assert_eq!(result.metadata.get("file").unwrap(), "test.py", 
                  "All results should be from file 'test.py'");
    }
}

#[test]
fn test_search_results_formatting() {
    // This test verifies that search results can be formatted as human-readable or JSON
    let results = vec![
        SearchResult {
            entity_id: "func1".to_string(),
            score: 0.95,
            metadata: {
                let mut map = HashMap::new();
                map.insert("type".to_string(), "function".to_string());
                map.insert("file".to_string(), "test.py".to_string());
                map
            },
        },
        SearchResult {
            entity_id: "class1".to_string(),
            score: 0.85,
            metadata: {
                let mut map = HashMap::new();
                map.insert("type".to_string(), "class".to_string());
                map.insert("file".to_string(), "test.py".to_string());
                map
            },
        },
    ];
    
    // Test human-readable format
    let human_output = format_search_results(&results, OutputFormat::Human);
    assert!(human_output.contains("func1"), "Human output should contain entity ID");
    assert!(human_output.contains("0.95"), "Human output should contain score");
    assert!(human_output.contains("function"), "Human output should contain entity type");
    assert!(human_output.contains("test.py"), "Human output should contain file path");
    
    // Test JSON format
    let json_output = format_search_results(&results, OutputFormat::Json);
    assert!(json_output.starts_with("["), "JSON output should start with [");
    assert!(json_output.ends_with("]"), "JSON output should end with ]");
    assert!(json_output.contains("\"entity_id\":\"func1\""), "JSON output should contain entity ID");
    assert!(json_output.contains("\"score\":0.95"), "JSON output should contain score");
}

#[test]
fn test_search_with_min_score_filter() {
    // This test verifies that search results can be filtered by minimum score
    let store = RedisVectorStore::new("redis://localhost:6379/0", "test_prefix");
    let query_vec = vec![1.0, 2.0, 3.0];
    
    // Set a high minimum score to filter results
    let options = SearchOptions {
        top_k: 10,
        min_score: Some(0.9),
        entity_types: None,
        file_filter: None,
    };
    
    let results = search_vectors(&store, &query_vec, &options);
    assert!(results.is_ok(), "Search with min score filter should succeed");
    let search_results = results.unwrap();
    
    // Verify all results have a score >= min_score
    for result in &search_results {
        assert!(result.score >= 0.9, 
                "All results should have a score >= min_score (0.9), got {}", result.score);
    }
}

#[test]
fn test_search_with_combined_filters() {
    // This test verifies that search results can be filtered by multiple criteria
    let store = RedisVectorStore::new("redis://localhost:6379/0", "test_prefix");
    let query_vec = vec![1.0, 2.0, 3.0];
    
    // Combine multiple filters
    let options = SearchOptions {
        top_k: 5,
        min_score: Some(0.7),
        entity_types: Some(vec!["function".to_string()]),
        file_filter: Some("test.py".to_string()),
    };
    
    let results = search_vectors(&store, &query_vec, &options);
    assert!(results.is_ok(), "Search with combined filters should succeed");
    let search_results = results.unwrap();
    
    // Verify all results match all filters
    for result in &search_results {
        assert!(result.score >= 0.7, 
                "All results should have a score >= 0.7");
        assert_eq!(result.metadata.get("type").unwrap(), "function", 
                  "All results should be of type 'function'");
        assert_eq!(result.metadata.get("file").unwrap(), "test.py", 
                  "All results should be from file 'test.py'");
    }
}
