//! Minimal RedisVectorStore struct and methods for TDD (dummy impl, no real Redis yet)

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_redis_vector_store_getters() {
        let store = RedisVectorStore::new("redis://localhost", "prefix");
        assert_eq!(store.redis_url(), "redis://localhost");
        assert_eq!(store.key_prefix(), "prefix");
    }
}
/// Trait for vector storage backends.
pub trait VectorStore {
    /// Upsert an embedding for an entity with optional file and type metadata.
    fn upsert_embedding(&self, entity_id: &str, embedding: &[f32], file: Option<&str>, entity_type: Option<&str>) -> Result<(), String>;
    /// Return top-k most similar embeddings to a query vector.
    fn similarity_search(&self, query: &[f32], top_k: usize) -> Vec<String>;
}

pub struct RedisVectorStore {
    redis_url: String,
    key_prefix: String,
}

impl RedisVectorStore {
    pub fn redis_url(&self) -> &str {
        &self.redis_url
    }
    pub fn key_prefix(&self) -> &str {
        &self.key_prefix
    }

    /// Dummy upsert_embedding for TDD (always Ok)
    pub fn upsert_embedding(&self, entity_id: &str, embedding: &[f32], file: Option<&str>, entity_type: Option<&str>) -> Result<(), String> {
        let _ = (entity_id, embedding, file, entity_type);
        Ok(())
    }

    /// Dummy similarity_search for TDD (returns dummy vec)
    pub fn similarity_search(&self, query: &[f32], top_k: usize) -> Vec<String> {
        let _ = (query, top_k);
        vec!["foo".to_string(), "bar".to_string(), "baz".to_string()][..top_k.min(3)].to_vec()
    }
}

impl RedisVectorStore {
    pub fn new(redis_url: &str, key_prefix: &str) -> Self {
        Self {
            redis_url: redis_url.to_string(),
            key_prefix: key_prefix.to_string(),
        }
    }
    pub fn upsert(&mut self, _entity_type: &str, _key: &str, _vector: &Vec<f32>) -> Result<(), &'static str> {
        // Dummy: always Ok
        Ok(())
    }
    pub fn query(&self, _entity_type: &str, _key: &str) -> Result<Vec<f32>, &'static str> {
        // Dummy: always return [1.0, 2.0, 3.0]
        Ok(vec![1.0, 2.0, 3.0])
    }
    pub fn make_key(&self, entity_type: &str, key: &str) -> String {
        format!("{}:{}:{}", self.key_prefix, entity_type, key)
    }
}

impl VectorStore for RedisVectorStore {
    fn upsert_embedding(&self, entity_id: &str, embedding: &[f32], file: Option<&str>, entity_type: Option<&str>) -> Result<(), String> {
        self.upsert_embedding(entity_id, embedding, file, entity_type)
    }
    fn similarity_search(&self, query: &[f32], top_k: usize) -> Vec<String> {
        self.similarity_search(query, top_k)
    }
}
