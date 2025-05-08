//! async redis operations for code_indexer_rust
//! - stores and queries files and entities
//! - uses fred 10.x async initialization:
//!   Config::from_url, Builder::from_config, client.init().await?

use fred::prelude::*;
use serde::{Serialize, Deserialize};
use crate::ast_parser::CodeEntity;
use std::collections::HashMap;
use std::time::Duration;

pub async fn create_redis_client(redis_url: &str) -> Result<RedisClient, fred::error::RedisError> {
    let config = Config::from_url(redis_url)?;
    let client = Builder::from_config(config)
        .with_connection_config(|cfg| {
            cfg.connection_timeout = Duration::from_secs(5);
        })
        .build()?;
    client.init().await?;
    Ok(client)
}

pub async fn store_file_content(redis: &RedisClient, key_prefix: &str, rel_path: &str, content: &str, size: usize, last_modified: i64) -> Result<(), fred::error::RedisError> {
    let file_data = serde_json::json!({
        "path": rel_path,
        "content": content,
        "size": size,
        "last_modified": last_modified
    });
    let file_key = format!("{}:files:{}", key_prefix, rel_path);
    redis.set(file_key, serde_json::to_string(&file_data)?).await?;
    redis.sadd(format!("{}:file_index", key_prefix), rel_path).await?;
    Ok(())
}

pub async fn store_code_entities(redis: &RedisClient, key_prefix: &str, entities: &[CodeEntity]) -> Result<(), fred::error::RedisError> {
    use serde_json::to_string;
    use std::collections::HashMap;
    let mut by_type: HashMap<&str, Vec<&CodeEntity>> = HashMap::new();
    for entity in entities {
        by_type.entry(&entity.entity_type).or_default().push(entity);
    }
    for (entity_type, ents) in by_type.iter() {
        let type_key = format!("{}:{}s", key_prefix, entity_type);
        let mut pipe = redis.pipeline();
        for entity in ents {
            let file_path = &entity.file_path;
            let name = &entity.name;
            let entity_id = if entity_type == &"method" {
                format!("{}:{}.{}", file_path, entity.parent_class.as_deref().unwrap_or("unknown"), name)
            } else {
                format!("{}:{}", file_path, name)
            };
            pipe.hset(&type_key, &entity_id, to_string(entity).unwrap());
            pipe.sadd(format!("{}:search_index:{}:{}", key_prefix, entity_type, name), &entity_id);
            pipe.sadd(format!("{}:file_entities:{}", key_prefix, file_path), format!("{}:{}", entity_type, &entity_id));
        }
        pipe.execute().await?;
    }
    Ok(())
}

pub async fn clear_file_data(redis: &RedisClient, key_prefix: &str, rel_paths: &[String]) -> Result<(), fred::error::RedisError> {
    for rel_path in rel_paths {
        let entities_key = format!("{}:file_entities:{}", key_prefix, rel_path);
        let entity_ids: Vec<String> = redis.smembers(&entities_key).await.unwrap_or_default();
        let mut pipe = redis.pipeline();
        for entity_id in entity_ids.iter() {
            let mut parts = entity_id.splitn(2, ':');
            let entity_type = parts.next().unwrap_or("");
            let id_part = parts.next().unwrap_or("");
            let type_key = format!("{}:{}s", key_prefix, entity_type);
            pipe.hdel(&type_key, id_part);
            let name = id_part.split(':').last().unwrap_or("");
            pipe.srem(format!("{}:search_index:{}:{}", key_prefix, entity_type, name), id_part);
        }
        pipe.del(&entities_key);
        pipe.del(format!("{}:files:{}", key_prefix, rel_path));
        pipe.srem(format!("{}:file_index", key_prefix), rel_path);
        pipe.execute().await?;
    }
    Ok(())
}

pub async fn query_code_entity(redis: &RedisClient, key_prefix: &str, entity_type: &str, name: Option<&str>) -> Result<Vec<CodeEntity>, fred::error::RedisError> {
    use serde_json::from_str;
    let mut results = Vec::new();
    if let Some(name) = name {
        let search_key = format!("{}:search_index:{}:{}", key_prefix, entity_type, name);
        let entity_ids: Vec<String> = redis.smembers(&search_key).await.unwrap_or_default();
        let type_key = format!("{}:{}s", key_prefix, entity_type);
        let mut pipe = redis.pipeline();
        for entity_id in &entity_ids {
            pipe.hget(&type_key, entity_id);
        }
        let entity_jsons: Vec<Option<String>> = pipe.execute().await?;
        for json_str in entity_jsons.into_iter().flatten() {
            if let Ok(entity) = from_str(&json_str) {
                results.push(entity);
            }
        }
    } else {
        let type_key = format!("{}:{}s", key_prefix, entity_type);
        let all_entities: HashMap<String, String> = redis.hgetall(&type_key).await.unwrap_or_default();
        for json_str in all_entities.values() {
            if let Ok(entity) = from_str(json_str) {
                results.push(entity);
            }
        }
    }
    Ok(results)
}

