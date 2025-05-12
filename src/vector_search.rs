//! Vector similarity search implementation
//! Provides functionality for searching vector embeddings with filtering and scoring

use crate::vector_store::VectorStore;
use std::collections::HashMap;
use log;
use serde::{Serialize, Deserialize};

/// Result of a vector similarity search
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResult {
    /// Entity identifier
    pub entity_id: String,
    /// Similarity score (0.0 to 1.0, higher is more similar)
    pub score: f32,
    /// Additional metadata about the entity
    pub metadata: HashMap<String, String>,
}

/// Options for vector similarity search
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchOptions {
    /// Maximum number of results to return
    pub top_k: usize,
    /// Minimum similarity score threshold (0.0 to 1.0)
    pub min_score: Option<f32>,
    /// Filter by entity types (e.g., "function", "class")
    pub entity_types: Option<Vec<String>>,
    /// Filter by file path
    pub file_filter: Option<String>,
}

/// Calculate cosine similarity between two vectors
fn cosine_similarity(a: &[f32], b: &[f32]) -> f32 {
    if a.len() != b.len() {
        log::warn!("Vector dimensions don't match: {} vs {}", a.len(), b.len());
        return 0.0;
    }

    let mut dot_product = 0.0;
    let mut a_norm = 0.0;
    let mut b_norm = 0.0;

    for i in 0..a.len() {
        dot_product += a[i] * b[i];
        a_norm += a[i] * a[i];
        b_norm += b[i] * b[i];
    }

    if a_norm == 0.0 || b_norm == 0.0 {
        return 0.0;
    }

    let similarity = dot_product / (a_norm.sqrt() * b_norm.sqrt());
    // Ensure the result is within [0, 1] range due to potential floating-point errors
    similarity.max(0.0).min(1.0)
}

/// Search for similar vectors with filtering options
///
/// # Arguments
/// * `store` - The vector store to search in
/// * `query` - The query vector to search for
/// * `options` - Search options including filters and limits
///
/// # Returns
/// * `Result<Vec<SearchResult>, String>` - Search results or error message
pub fn search_vectors(
    store: &impl VectorStore,
    query: &[f32],
    options: &SearchOptions,
) -> Result<Vec<SearchResult>, String> {
    log::info!(
        "Performing vector search with top_k={}, min_score={:?}, entity_types={:?}, file_filter={:?}",
        options.top_k,
        options.min_score,
        options.entity_types,
        options.file_filter
    );

    // Get all entity IDs from the store
    let entity_ids = store.get_all_entity_ids()?;
    
    // Collect entity vectors and metadata
    let mut results = Vec::new();
    for entity_id in entity_ids {
        // Get entity vector
        let entity_vector = match store.get_entity_vector(&entity_id) {
            Ok(vector) => vector,
            Err(e) => {
                log::warn!("Failed to get vector for entity {}: {}", entity_id, e);
                continue;
            }
        };
        
        // Get entity metadata
        let metadata = match store.get_entity_metadata(&entity_id) {
            Ok(meta) => meta,
            Err(e) => {
                log::warn!("Failed to get metadata for entity {}: {}", entity_id, e);
                continue;
            }
        };
        
        // Apply entity type filter if specified
        if let Some(ref entity_types) = options.entity_types {
            if let Some(entity_type) = metadata.get("type") {
                if !entity_types.contains(&entity_type.to_string()) {
                    continue;
                }
            } else {
                continue;
            }
        }
        
        // Apply file filter if specified
        if let Some(ref file_filter) = options.file_filter {
            if let Some(file) = metadata.get("file") {
                if file != file_filter {
                    continue;
                }
            } else {
                continue;
            }
        }
        
        // Calculate similarity score
        let score = cosine_similarity(query, &entity_vector);
        
        // Apply minimum score filter if specified
        if let Some(min_score) = options.min_score {
            if score < min_score {
                continue;
            }
        }
        
        // Add to results
        results.push(SearchResult {
            entity_id,
            score,
            metadata,
        });
    }
    
    // Sort results by score in descending order
    results.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap_or(std::cmp::Ordering::Equal));
    
    // Limit to top_k results
    let results: Vec<SearchResult> = results.into_iter().take(options.top_k).collect();
    
    log::info!("Vector search returned {} results", results.len());
    Ok(results)
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_cosine_similarity() {
        // Test with identical vectors
        let a = vec![1.0, 2.0, 3.0];
        let b = vec![1.0, 2.0, 3.0];
        let similarity = cosine_similarity(&a, &b);
        assert!((similarity - 1.0).abs() < 1e-6, "Expected similarity close to 1.0, got {}", similarity);
        
        // Test with orthogonal vectors
        let a = vec![1.0, 0.0, 0.0];
        let b = vec![0.0, 1.0, 0.0];
        let similarity = cosine_similarity(&a, &b);
        assert!((similarity - 0.0).abs() < 1e-6, "Expected similarity close to 0.0, got {}", similarity);
        
        // Test with vectors of different lengths
        let a = vec![1.0, 2.0, 3.0];
        let b = vec![1.0, 2.0];
        let similarity = cosine_similarity(&a, &b);
        assert!((similarity - 0.0).abs() < 1e-6, "Expected similarity close to 0.0, got {}", similarity);
    }
}
