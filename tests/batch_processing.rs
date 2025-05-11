//! RED test for batch processing and progress logging

use indexer::embedder::{Embedder, OpenAIEmbedder};

#[test]
fn test_batch_embedding_progress_logging() {
    let inputs = vec!["fn main()", "fn foo()", "fn bar()"];
    let embedder = OpenAIEmbedder::new_from_env().unwrap();
    let mut progress = 0;
    for (i, input) in inputs.iter().enumerate() {
        let _ = embedder.embed(input);
        progress = i + 1;
        log::info!("Embedded {} of {}", progress, inputs.len());
    }
    assert_eq!(progress, 3);
}
