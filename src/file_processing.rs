//! file collection and filtering logic for code_indexer_rust
//! - traverses directories
//! - respects ignore patterns

use ignore::{WalkBuilder, DirEntry};
use std::path::{Path, PathBuf};

const SKIP_DIRS: &[&str] = &[".logs", ".venv", ".git", "__pycache__", "node_modules", "build", "dist"];

pub fn collect_python_files(app_dir: &Path, specific_files: Option<&[String]>) -> Vec<PathBuf> {
    if let Some(files) = specific_files {
        return files.iter()
            .map(|f| app_dir.join(f))
            .filter(|p| p.exists() && p.is_file() && p.extension().map(|e| e == "py").unwrap_or(false))
            .collect();
    }
    let mut files = Vec::new();
    let walker = WalkBuilder::new(app_dir)
        .hidden(false)
        .ignore(true)
        .git_ignore(true)
        .filter_entry(|e| !should_skip(e))
        .build();
    for entry in walker {
        if let Ok(entry) = entry {
            if entry.path().extension().map(|e| e == "py").unwrap_or(false) {
                files.push(entry.into_path());
            }
        }
    }
    files
}

fn should_skip(entry: &DirEntry) -> bool {
    let path = entry.path();
    if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
        SKIP_DIRS.contains(&name)
    } else {
        false
    }
}
