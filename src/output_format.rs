//! Output formatting utilities for search results

use serde_json;

pub fn format_human_readable(results: &Vec<(&str, f32)>) -> String {
    results.iter().map(|(id, score)| format!("{}: {}", id, score)).collect::<Vec<_>>().join("\n")
}

pub fn format_json(results: &Vec<(&str, f32)>) -> String {
    serde_json::to_string(&results.iter().map(|(id, score)| (id.to_string(), score)).collect::<Vec<_>>()).unwrap()
}
