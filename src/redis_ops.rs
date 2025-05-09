//! async redis operations for code_indexer_rust
//! - stores and queries files and entities
//! - uses fred 10.x async initialization:
//!   Config::from_url, Builder::from_config, client.init().await?

use fred::prelude::*; // For Client, Config, Builder, Error, Expiration, SetOptions, etc.

// Assuming these are still needed by your logic.
// The 'unused' warning for Serialize/Deserialize here will appear if CodeEntity (defined elsewhere)
// is the only serializable type and has its own `use serde::...` for the derive.

use crate::ast_parser::CodeEntity;
use std::collections::HashMap;
use std::time::Duration;

// This function was already mostly correct in your provided snippet based on previous iterations.
pub async fn create_redis_client(redis_url: &str) -> Result<Client, Error> {
    let config = Config::from_url(redis_url)?;
    let client = Builder::from_config(config)
        .with_connection_config(|cfg| {
            cfg.connection_timeout = Duration::from_secs(5);
        })
        .build()?;
    client.init().await?;
    Ok(client)
}

pub async fn store_file_content(
    redis: &Client, // Changed from &RedisClient
    key_prefix: &str,
    rel_path: &str,
    content: &str,
    size: usize,
    last_modified: i64,
) -> Result<(), Error> { // Changed from fred::error::Error
    let file_data = serde_json::json!({
        "path": rel_path,
        "content": content,
        "size": size,
        "last_modified": last_modified
    });
    let file_key = format!("{}:files:{}", key_prefix, rel_path);

    // 1. Handle serde_json::Error to fred::Error conversion
    let value_to_set = serde_json::to_string(&file_data).map_err(|e| {
        Error::new(
            ErrorKind::Parse, // Or ErrorKind::Other if more appropriate
            format!("Failed to serialize file_data for {}: {}", rel_path, e),
        )
    })?;

    // 2. Call redis.set with the correct 5 arguments for fred 10.1.0
    //    (key, value, expiration: Option<Expiration>, options: Option<SetOptions>, get: bool)
    let _: String = redis.set(file_key, value_to_set, None, None, false).await?;
    let _: u64 = redis.sadd(format!("{}:file_index", key_prefix), rel_path).await?;
    Ok(())
}

pub async fn store_code_entities(
    redis: &Client, // Changed from &RedisClient
    key_prefix: &str,
    entities: &[CodeEntity],
) -> Result<(), Error> { // Changed from fred::error::Error
    use serde_json::to_string; // Local import is fine for clarity
    // HashMap is imported at the top

    let mut by_type: HashMap<&str, Vec<&CodeEntity>> = HashMap::new();
    for entity in entities {
        by_type.entry(&entity.entity_type).or_default().push(entity);
    }

    for (entity_type, ents) in by_type.iter() {
        let type_key = format!("{}:{}s", key_prefix, entity_type);
        let pipe = redis.pipeline();
        for entity in ents {
            let entity_id = &entity.name;
            let value_str = match to_string(entity) {
                Ok(val) => val,
                Err(e) => {
                    return Err(Error::new(
                        ErrorKind::Parse,
                        format!("Failed to serialize entity {}: {}", entity_id, e),
                    ));
                }
            };
            let _: u64 = pipe.hset(&type_key, (entity_id, &value_str)).await?;
            let _: u64 = pipe.sadd(format!("{}:search_index:{}:{}", key_prefix, entity_type, entity.name), entity_id).await?;
            let _: u64 = pipe.sadd(format!("{}:file_entities:{}", key_prefix, entity.file_path), format!("{}:{}", entity_type, entity_id)).await?;
        }
        let _: Vec<Value> = pipe.all().await?;
    }
    Ok(())
}

pub async fn clear_file_data(
    redis: &Client,
    key_prefix: &str,
    rel_paths: &[String],
) -> Result<(), Error> {
    for rel_path in rel_paths {
        let entities_key = format!("{}:file_entities:{}", key_prefix, rel_path);
        let entity_ids: Vec<String> = redis.smembers(&entities_key).await.unwrap_or_default();
        let pipe = redis.pipeline();
        for entity_id in entity_ids.iter() {
            let mut parts = entity_id.splitn(2, ':');
            let entity_type = parts.next().unwrap_or("");
            let id_part = parts.next().unwrap_or("");
            let type_key = format!("{}:{}s", key_prefix, entity_type);
            let _: u64 = pipe.hdel(&type_key, id_part).await?;
            let name = id_part.split(':').last().unwrap_or("");
            let _: u64 = pipe
                .srem(
                    format!("{}:search_index:{}:{}", key_prefix, entity_type, name),
                    id_part,
                )
                .await?;
        }

        let _: u64 = pipe.del(&entities_key).await?;
        let _: u64 = pipe.del(format!("{}:files:{}", key_prefix, rel_path)).await?;
        let _: u64 = pipe.srem(format!("{}:file_index", key_prefix), rel_path).await?;

        // execute the pipeline for this rel_path
        let _: Vec<Value> = pipe.all().await?;
    }  // ← closes the for-loop

    Ok(())  // ← now return success after all paths processed
}  // ← closes clear_file_data


// Now starts the next function:
pub async fn query_code_entity(
    redis: &Client, // Changed from &RedisClient
    key_prefix: &str,
    entity_type: &str,
    name: Option<&str>,
) -> Result<Vec<CodeEntity>, Error> { // Changed from fred::error::Error
    use serde_json::from_str; // Local import is fine

    let mut results = Vec::new();
    if let Some(name_val) = name { // Renamed to avoid conflict if `name` is a field, good practice
        let search_key = format!("{}:search_index:{}:{}", key_prefix, entity_type, name_val);
        let entity_ids: Vec<String> = redis.smembers(&search_key).await.unwrap_or_default();
        let type_key = format!("{}:{}s", key_prefix, entity_type);

        if !entity_ids.is_empty() { // Good optimization
            let pipe = redis.pipeline();
            for entity_id in &entity_ids {
                let _: () = pipe.hget(&type_key, entity_id).await?;
            }
            
            let hget_results: Vec<Result<Option<String>, Error>> = pipe.try_all().await;
            for hget_result in hget_results {
                let json_opt = hget_result?;
                if let Some(json_str) = json_opt {
                    if let Ok(entity) = from_str(&json_str) {
                        results.push(entity);
                    }
                }
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