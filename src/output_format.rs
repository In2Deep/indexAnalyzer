//! Output formatting utilities for search results
//! Provides formatting options for vector search results

use crate::vector_search::SearchResult;
use serde_json;
use log;

/// Output format options for search results
#[derive(Debug, Clone, Copy)]
pub enum OutputFormat {
    /// Human-readable text format
    Human,
    /// JSON format
    Json,
}

/// Format search results according to the specified output format
///
/// # Arguments
/// * `results` - Vector of search results
/// * `format` - Output format (Human or Json)
///
/// # Returns
/// * `String` - Formatted output
pub fn format_search_results(results: &[SearchResult], format: OutputFormat) -> String {
    log::info!("Formatting {} search results as {:?}", results.len(), format);
    
    match format {
        OutputFormat::Human => format_human_readable_search_results(results),
        OutputFormat::Json => format_json_search_results(results),
    }
}

// Original functions for classic search results
pub fn format_human_readable(results: &Vec<(&str, f32)>) -> String {
    results.iter().map(|(id, score)| format!("{}: {}", id, score)).collect::<Vec<_>>().join("\n")
}

pub fn format_json(results: &Vec<(&str, f32)>) -> String {
    serde_json::to_string(&results.iter().map(|(id, score)| (id.to_string(), score)).collect::<Vec<_>>()).unwrap_or_else(|_| String::from("[]"))
}

// New functions for SearchResult type
pub fn format_human_readable_search_results(results: &[SearchResult]) -> String {
    if results.is_empty() {
        return "No results found.".to_string();
    }
    
    let mut output = String::from("Results:\n");
    
    for (i, result) in results.iter().enumerate() {
        let metadata_str = result.metadata.iter()
            .map(|(k, v)| format!("{}: {}", k, v))
            .collect::<Vec<_>>()
            .join(", ");
            
        output.push_str(&format!(
            "{}. {} (score: {:.4}) - {}",
            i + 1,
            result.entity_id,
            result.score,
            metadata_str
        ));
        
        if i < results.len() - 1 {
            output.push_str("\n");
        }
    }
    
    output
}

pub fn format_json_search_results(results: &[SearchResult]) -> String {
    match serde_json::to_string(results) {
        Ok(json) => json,
        Err(e) => {
            log::error!("Failed to serialize search results to JSON: {}", e);
            String::from("[]")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    
    #[test]
    fn test_format_human_readable_classic() {
        let results = vec![("func1", 0.95f32), ("class1", 0.85f32)];
        let output = format_human_readable(&results);
        assert!(output.contains("func1"));
        assert!(output.contains("0.95"));
    }
    
    #[test]
    fn test_format_json_classic() {
        let results = vec![("func1", 0.95f32)];
        let output = format_json(&results);
        assert!(output.contains("func1"));
        assert!(output.contains("0.95"));
    }
    
    #[test]
    fn test_format_human_readable_search_results() {
        let mut metadata1 = HashMap::new();
        metadata1.insert("type".to_string(), "function".to_string());
        metadata1.insert("file".to_string(), "test.py".to_string());
        
        let mut metadata2 = HashMap::new();
        metadata2.insert("type".to_string(), "class".to_string());
        metadata2.insert("file".to_string(), "test.py".to_string());
        
        let results = vec![
            SearchResult {
                entity_id: "func1".to_string(),
                score: 0.95,
                metadata: metadata1,
            },
            SearchResult {
                entity_id: "class1".to_string(),
                score: 0.85,
                metadata: metadata2,
            },
        ];
        
        let output = format_human_readable_search_results(&results);
        assert!(output.contains("func1"));
        assert!(output.contains("0.95"));
        assert!(output.contains("function"));
    }
    
    #[test]
    fn test_format_json_search_results() {
        let mut metadata = HashMap::new();
        metadata.insert("type".to_string(), "function".to_string());
        
        let results = vec![
            SearchResult {
                entity_id: "func1".to_string(),
                score: 0.95,
                metadata,
            },
        ];
        
        let output = format_json_search_results(&results);
        assert!(output.contains("func1"));
        assert!(output.contains("0.95"));
        assert!(output.contains("function"));
    }
    
    #[test]
    fn test_empty_search_results() {
        let results: Vec<SearchResult> = vec![];
        
        let human_output = format_human_readable_search_results(&results);
        assert_eq!(human_output, "No results found.");
        
        let json_output = format_json_search_results(&results);
        assert_eq!(json_output, "[]");
    }
}
