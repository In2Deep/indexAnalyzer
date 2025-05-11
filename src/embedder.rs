//! Embedder trait and mock/test implementation

pub trait Embedder {
    fn embed(&self, input: &str) -> Vec<f32>;
}

pub struct OpenAIEmbedder {
    api_key: String,
}

impl OpenAIEmbedder {
    pub fn api_key(&self) -> &str {
        &self.api_key
    }

}

impl OpenAIEmbedder {
    pub fn new_from_env() -> Result<Self, &'static str> {
        match std::env::var("OPENAI_API_KEY") {
            Ok(key) => Ok(Self { api_key: key }),
            Err(_) => Err("OPENAI_API_KEY not set"),
        }
    }
}

impl Embedder for OpenAIEmbedder {
    fn embed(&self, input: &str) -> Vec<f32> {
        log::info!("embedding input with OpenAI: {}", input);
        vec![1.0, 2.0, 3.0] // dummy
    }
}

pub struct HFEmbedder {
    api_key: String,
}

impl HFEmbedder {
    pub fn api_key(&self) -> &str {
        &self.api_key
    }

}

impl HFEmbedder {
    pub fn new_from_env() -> Result<Self, &'static str> {
        match std::env::var("HF_API_KEY") {
            Ok(key) => Ok(Self { api_key: key }),
            Err(_) => Err("HF_API_KEY not set"),
        }
    }
}

impl Embedder for HFEmbedder {
    fn embed(&self, _input: &str) -> Vec<f32> {
        vec![1.0, 2.0, 3.0] // dummy
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_openai_api_key_getter() {
        let embedder = OpenAIEmbedder { api_key: "testkey".to_string() };
        assert_eq!(embedder.api_key(), "testkey");
    }
    #[test]
    fn test_hf_api_key_getter() {
        let embedder = HFEmbedder { api_key: "testkey2".to_string() };
        assert_eq!(embedder.api_key(), "testkey2");
    }

    use super::*;
    struct MockEmbedder;
    impl Embedder for MockEmbedder {
        fn embed(&self, _input: &str) -> Vec<f32> {
            vec![0.0, 1.0, 2.0]
        }
    }
    #[test]
    fn test_mock_embedder_trait() {
        let embedder = MockEmbedder;
        let vec = embedder.embed("foo");
        assert_eq!(vec, vec![0.0, 1.0, 2.0]);
    }
}
