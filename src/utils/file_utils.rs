// File utility functions

use anyhow::{Context, Result};
use std::path::{Path, PathBuf};

/// Find all component files in a directory
pub async fn find_component_files<P: AsRef<Path>>(
    dir: P,
    extension: &str,
) -> Result<Vec<PathBuf>> {
    let mut result = Vec::new();
    let mut entries = tokio::fs::read_dir(dir).await?;
    
    while let Some(entry) = entries.next_entry().await? {
        let path = entry.path();
        
        if path.is_dir() {
            // Recursively search subdirectories
            let mut subdir_results = find_component_files(&path, extension).await?;
            result.append(&mut subdir_results);
        } else if let Some(ext) = path.extension() {
            if ext == extension {
                result.push(path);
            }
        }
    }
    
    Ok(result)
}

/// Read a component file
pub async fn read_component_file<P: AsRef<Path>>(path: P) -> Result<String> {
    tokio::fs::read_to_string(path)
        .await
        .context("Failed to read component file")
}

/// Write a component file
pub async fn write_component_file<P: AsRef<Path>>(
    path: P,
    content: &str,
) -> Result<()> {
    // Create parent directory if it doesn't exist
    if let Some(parent) = path.as_ref().parent() {
        tokio::fs::create_dir_all(parent).await?;
    }
    
    tokio::fs::write(path, content)
        .await
        .context("Failed to write component file")
}
