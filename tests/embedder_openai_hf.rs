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
    env::set_var("HF_API_KEY", "hf-test");
    let embedder = HFEmbedder::new_from_env().unwrap();
    let vec = embedder.embed("foo");
    assert_eq!(vec, vec![1.0, 2.0, 3.0]); // placeholder for real embedding
}

#[test]
fn test_openai_embedder_missing_key() {
    env::remove_var("OPENAI_API_KEY");
    let result = OpenAIEmbedder::new_from_env();
    assert!(result.is_err());
}

#[test]
fn test_hf_embedder_missing_key() {
    env::remove_var("HF_API_KEY");
    let result = HFEmbedder::new_from_env();
    assert!(result.is_err());
}
