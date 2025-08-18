use crate::errors::{DitResult, FsError};
use std::path::{Path, PathBuf};
use std::fs;


/// Returns the current working directory
pub fn get_cwd() -> DitResult<PathBuf> {
    Ok(std::env::current_dir()?)
}


/// Converts a path to a string
pub fn path_to_string(path: &Path) -> String {
    path.to_string_lossy().to_string()
}


/// Resolves a given path to an absolute, canonical path.
///
/// - Follows symbolic links.
/// - Returns an error if the path does not exist.
/// - On Windows, strips extended-length path prefix (e.g. `\\?\C:\...`)
pub fn resolve_absolute_path(input: &Path) -> DitResult<PathBuf> {
    let canonical = fs::canonicalize(input)
        .map_err(|_| FsError::AbsPathResolveError(path_to_string(input)))?;

    Ok(normalize_path(canonical))
}

#[cfg(windows)]
fn normalize_path(p: PathBuf) -> PathBuf {
    // Remove extended-length prefix like \\?\C:\...
    if let Ok(s) = p.into_os_string().into_string() {
        let cleaned = s.strip_prefix(r"\\?\").unwrap_or(&s);
        PathBuf::from(cleaned)
    } else {
        // fallback for non-UTF-8 paths
        PathBuf::new()
    }
}

#[cfg(not(windows))]
fn normalize_path(p: PathBuf) -> PathBuf {
    p
}
