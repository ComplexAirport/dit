use crate::errors::{DitResult, FsError};
use std::fs;
use std::fs::File;
use std::path::Path;

/// Removes a file using [`fs::remove_file`] and maps the error to [`FsError`]
pub fn remove_file<P: AsRef<Path>>(path: P) -> DitResult<()> {
    let path = path.as_ref();
    fs::remove_file(path)
        .map_err(|_| FsError::FileRemoveError(path.display().to_string()).into())
}


/// Removes a directory using [`fs::remove_dir_all`] and maps the error to [`FsError`]
pub fn remove_dir<P: AsRef<Path>>(path: P) -> DitResult<()> {
    let path = path.as_ref();
    fs::remove_dir_all(path)
        .map_err(|_| FsError::DirRemoveError(path.display().to_string()).into())
}


/// Creates a file and all the necessary subdirectories (if they don't exist) and maps
/// the result to [`FsError`]
pub fn create_file_all<P: AsRef<Path>>(path: P) -> DitResult<()> {
    let path = path.as_ref();

    if path.is_file() {
        return Ok(());
    }

    if let Some(parent) = Path::new(path).parent() {
        fs::create_dir_all(parent)
            .map_err(|_| FsError::DirCreateError(parent.display().to_string()))?;
    }

    File::create(path)
        .map_err(|_| FsError::FileCreateError(path.display().to_string()))?;

    Ok(())
}

