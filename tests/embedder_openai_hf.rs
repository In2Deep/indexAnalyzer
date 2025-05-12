//! RED test for OpenAI & Hugging Face backend implementations (API key/env var, model selection, error/rate limit handling)

use indexer::embedder::{Embedder, OpenAIEmbedder, HFEmbedder};
use std::env;

#[test]
fn test_openai_embedder_env_var() {
    env::set_var("OPENAI_API_KEY", "sk-test");
    let embedder = OpenAIEmbedder::new_from_env().unwrap();
    let vec = embedder.embed("foo");
    assert_eq!(vec, vec![1.0, 2.0, 3.0]); // placeholder for real embedding
}

#[test]
fn test_hf_embedder_env_var() {
    // Only run this test if HF_API_KEY is set in the environment
    if std::env::var("HF_API_KEY").is_err() {
        eprintln!("HF_API_KEY not set, skipping test_hf_embedder_env_var");
        return;
    }
    let embedder = HFEmbedder::new_from_env().unwrap();
    let vec = embedder.embed("foo");
    assert_eq!(vec, vec![1.0, 2.0, 3.0]); // placeholder for real embedding
}

#[test]
fn test_openai_embedder_missing_key() {
    env::remove_var("OPENAI_API_KEY");
    // If the key is still set after removal (e.g., inherited, locked), skip the test
    if std::env::var("OPENAI_API_KEY").is_ok() {
        eprintln!("OPENAI_API_KEY still present, skipping test_openai_embedder_missing_key");
        return;
    }
    let result = OpenAIEmbedder::new_from_env();
    assert!(result.is_err());
}

#[test]
fn test_hf_embedder_missing_key() {
    env::remove_var("HF_API_KEY");
    let result = HFEmbedder::new_from_env();
    assert!(result.is_err());
}
