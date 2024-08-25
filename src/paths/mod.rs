use anyhow::{Context, Result};
use std::path::PathBuf;

/// Canonicalizes a list of file paths.
///
/// Takes a slice of strings representing file paths and returns a vector of
/// canonicalized `PathBuf` objects. If any path cannot be canonicalized, an
/// error is returned.
pub fn canonicalize_paths(paths: &[String]) -> Result<Vec<PathBuf>> {
    let mut full_paths = Vec::new();
    for path in paths {
        let full_path = PathBuf::from(path.clone())
            .canonicalize()
            .with_context(|| format!("Failed to canonicalize path: {:?}", path))?;
        log::info!("Full path: {}", full_path.display());
        full_paths.push(full_path);
    }
    Ok(full_paths)
}
