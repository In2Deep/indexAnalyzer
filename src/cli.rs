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
        #[arg(default_value = ".")]
        path: String,
    },
    /// update specific files in memory
    Refresh {
        files: String,
        #[arg(long)]
        project: Option<String>,
    },
    /// search for code in memory
    Recall {
        entity_type: String,
        name: Option<String>,
        #[arg(long)]
        project: Option<String>,
    },
    /// check what's in memory
    Status {
        #[arg(long)]
        project: Option<String>,
    },
    /// clear indexed data
    Forget {
        #[arg(long, default_value = ".")]
        project: String,
    },
}
