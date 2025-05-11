//! Tests for vectorize subcommand CLI parsing (TDD: Phase 2 - RED)

use clap::Parser;
use indexer::cli::{CliArgs, Commands};

#[test]
fn test_vectorize_parsing_mandatory_args() {
    // Simulate CLI: indexer vectorize --name my_project --path ./src
    let args = vec![
        "indexer",
        "vectorize",
        "--name",
        "my_project",
        "--path",
        "./src",
    ];
    let cli = CliArgs::parse_from(args);
    match cli.command {
        Commands::Vectorize { name, path, provider, db, batch_size, dry_run, verbose } => {
            assert_eq!(name, "my_project");
            assert_eq!(path, "./src");
            assert!(provider.is_none());
            assert!(db.is_none());
            assert!(batch_size.is_none());
            assert!(!dry_run);
            assert!(!verbose);
        }
        _ => panic!("Expected vectorize subcommand to be parsed"),
    }
}

#[test]
fn test_vectorize_parsing_provider_arg() {
    // Case 1: --provider present
    let args = vec![
        "indexer",
        "vectorize",
        "--name",
        "my_project",
        "--path",
        "./src",
        "--provider",
        "openai_special",
    ];
    let cli = CliArgs::parse_from(args);
    match cli.command {
        Commands::Vectorize { provider, .. } => {
            assert_eq!(provider, Some("openai_special".to_string()));
        }
        _ => panic!("Expected vectorize subcommand to be parsed"),
    }

    // Case 2: --provider absent
    let args = vec![
        "indexer",
        "vectorize",
        "--name",
        "my_project",
        "--path",
        "./src",
    ];
    let cli = CliArgs::parse_from(args);
    match cli.command {
        Commands::Vectorize { provider, .. } => {
            assert!(provider.is_none());
        }
        _ => panic!("Expected vectorize subcommand to be parsed"),
    }
}

#[test]
fn test_vectorize_parsing_db_arg() {
    // Case 1: --db present
    let args = vec![
        "indexer",
        "vectorize",
        "--name",
        "my_project",
        "--path",
        "./src",
        "--db",
        "redis_special",
    ];
    let cli = CliArgs::parse_from(args);
    match cli.command {
        Commands::Vectorize { db, .. } => {
            assert_eq!(db, Some("redis_special".to_string()));
        }
        _ => panic!("Expected vectorize subcommand to be parsed"),
    }

    // Case 2: --db absent
    let args = vec![
        "indexer",
        "vectorize",
        "--name",
        "my_project",
        "--path",
        "./src",
    ];
    let cli = CliArgs::parse_from(args);
    match cli.command {
        Commands::Vectorize { db, .. } => {
            assert!(db.is_none());
        }
        _ => panic!("Expected vectorize subcommand to be parsed"),
    }
}