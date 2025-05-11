//! Tests for vector-recall subcommand CLI parsing (TDD: Phase 2 - RED)

use clap::Parser;
use indexer::cli::{CliArgs, Commands};

#[test]
fn test_vector_recall_parsing_required_args() {
    // Simulate CLI: indexer vector-recall --name my_project --query "foo bar"
    let args = vec![
        "indexer",
        "vector-recall",
        "--name",
        "my_project",
        "--query",
        "foo bar",
    ];
    let cli = CliArgs::parse_from(args);
    match cli.command {
        Commands::VectorRecall { name, query, provider, db, top_k, json } => {
            assert_eq!(name, "my_project");
            assert_eq!(query, "foo bar");
            assert!(provider.is_none());
            assert!(db.is_none());
            assert!(top_k.is_none());
            assert!(!json);
        }
        _ => panic!("Expected vector-recall subcommand to be parsed"),
    }
}

#[test]
fn test_vector_recall_missing_required_args() {
    // Missing --query
    let args = vec![
        "indexer",
        "vector-recall",
        "--name",
        "my_project",
    ];
    let result = CliArgs::try_parse_from(args);
    assert!(result.is_err(), "Should error if --query is missing");

    // Missing --name
    let args = vec![
        "indexer",
        "vector-recall",
        "--query",
        "foo bar",
    ];
    let result = CliArgs::try_parse_from(args);
    assert!(result.is_err(), "Should error if --name is missing");
}

#[test]
fn test_vector_recall_optional_args() {
    // All optional args present
    let args = vec![
        "indexer",
        "vector-recall",
        "--name",
        "my_project",
        "--query",
        "foo bar",
        "--provider",
        "openai",
        "--db",
        "redis",
        "--top-k",
        "7",
        "--json",
    ];
    let cli = CliArgs::parse_from(args);
    match cli.command {
        Commands::VectorRecall { provider, db, top_k, json, .. } => {
            assert_eq!(provider, Some("openai".to_string()));
            assert_eq!(db, Some("redis".to_string()));
            assert_eq!(top_k, Some(7));
            assert!(json);
        }
        _ => panic!("Expected vector-recall subcommand to be parsed"),
    }
}
