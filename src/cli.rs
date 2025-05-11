//! cli argument parsing for code_indexer_rust

use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(
    name = "code_indexer_rust",
    about = "ai's external brain cells (rust edition)",
    version = "0.1.0"
)]
pub struct CliArgs {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// index code in a directory
    Remember {
        #[arg(long = "name", alias = "project-name")]
        name: String,
        #[arg(default_value = ".")]
        path: String,
    },
    /// update specific files in memory
    Refresh {
        #[arg(long = "name", alias = "project-name")]
        name: String,
        files: String,
    },
    /// search for code in memory
    Recall {
        #[arg(long)]
        entity: Option<String>,
        #[arg(long = "show-lines")]
        show_lines: bool,
        #[arg(long)]
        max: Option<usize>,
        #[arg(long = "name", alias = "project-name")]
        project_name: String,
    },
    /// check what's in memory
    Status {
        #[arg(long = "name", alias = "project-name")]
        name: String,
    },
    /// clear indexed data
    Forget {
        #[arg(long = "name", alias = "project-name")]
        name: String,
    },
    /// batch-generate embeddings for code entities
    Vectorize {
        #[arg(long = "name", alias = "project-name")]
        name: String,
        #[arg(long = "path")]
        path: String,
        #[arg(long = "provider")]
        provider: Option<String>,
        #[arg(long = "db")]
        db: Option<String>,
        #[arg(long = "batch-size")]
        batch_size: Option<usize>,
        #[arg(long = "dry-run")]
        dry_run: bool,
        #[arg(long = "verbose")]
        verbose: bool,
    },
}
