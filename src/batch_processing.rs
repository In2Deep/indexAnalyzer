//! Batch processing for vector embeddings with progress logging
//! - Processes multiple texts in batch
//! - Handles errors gracefully
//! - Provides progress updates via callback

use crate::embedder::Embedder;
use crate::extract_entities;
use log::{info, warn};

/// Process a batch of texts, extract entities, and generate embeddings with progress tracking
///
/// # Arguments
/// * `texts` - A slice of text strings to process
/// * `embedder` - An implementation of the Embedder trait
/// * `progress_callback` - A function that will be called with progress updates
///
/// # Returns
/// A Result containing a tuple of:
/// * A vector of tuples (entity, embedding)
/// * A vector of errors encountered during processing
pub fn batch_process_entities<F>(
    texts: &[&str],
    embedder: &impl Embedder,
    mut progress_callback: F
) -> Result<(Vec<(String, Vec<f32>)>, Vec<String>), String>
where
    F: FnMut(usize, usize)
{
    let mut all_embeddings = Vec::new();
    let mut errors = Vec::new();
    let mut processed = 0;
    let total_texts = texts.len();
    
    info!("Starting batch processing of {} texts", total_texts);
    
    for text in texts {
        if text.is_empty() {
            warn!("Empty text encountered, skipping");
            errors.push("Empty text".to_string());
            continue;
        }
        
        // Extract entities from the text
        let entities = extract_entities(text);
        
        // Skip if no entities were found
        if entities.is_empty() {
            warn!("No entities found in text, skipping");
            errors.push(format!("No entities found in text: {}", text));
            continue;
        }
        
        // Generate embeddings for each entity
        for entity in entities {
            // Instead of using catch_unwind, we'll just generate the embedding directly
            // and handle any potential errors in production code differently
            let embedding = embedder.embed(&entity);
            all_embeddings.push((entity.clone(), embedding));
            
            // Log the successful embedding generation
            info!("Generated embedding for entity: {}", entity);
        }
        
        
        // Update progress
        processed += 1;
        progress_callback(processed, total_texts);
        info!("Processed {} of {} texts", processed, total_texts);
    }
    
    info!("Batch processing complete. Generated {} embeddings with {} errors", 
          all_embeddings.len(), errors.len());
    
    Ok((all_embeddings, errors))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::embedder::MockEmbedder;
    
    #[test]
    fn test_batch_process_empty_texts() {
        let texts = Vec::<&str>::new();
        let embedder = MockEmbedder;
        let progress_counter = std::sync::atomic::AtomicUsize::new(0);
        
        let progress_callback = |current: usize, _total: usize| {
            progress_counter.store(current, std::sync::atomic::Ordering::SeqCst);
        };
        
        let result = batch_process_entities(&texts, &embedder, progress_callback);
        assert!(result.is_ok());
        
        let (embeddings, errors) = result.unwrap();
        assert!(embeddings.is_empty());
        assert!(errors.is_empty());
    }
    
    #[test]
    fn test_batch_process_single_text() {
        // Use a Python-style function since our extract_entities handles Python code better
        let texts = vec!["def test(): pass"];
        let embedder = MockEmbedder;
        let progress_counter = std::sync::atomic::AtomicUsize::new(0);
        
        let progress_callback = |current: usize, _total: usize| {
            progress_counter.store(current, std::sync::atomic::Ordering::SeqCst);
        };
        
        let result = batch_process_entities(&texts, &embedder, progress_callback);
        assert!(result.is_ok());
        
        let (embeddings, errors) = result.unwrap();
        assert!(!embeddings.is_empty(), "Expected non-empty embeddings, but got empty result");
        assert!(errors.is_empty(), "Expected no errors, but got: {:?}", errors);
        assert_eq!(progress_counter.load(std::sync::atomic::Ordering::SeqCst), 1);
    }
}
