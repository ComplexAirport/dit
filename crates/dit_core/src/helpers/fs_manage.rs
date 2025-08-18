use crate::helpers::path_to_string;
use crate::errors::{DitResult, FsError};
use std::fs::{self, File};
use std::path::Path;
use std::io;

/// Removes a file using [`fs::remove_file`] if it exists
/// and maps the error to [`FsError`]
pub fn remove_file_if_exists(path: &Path) -> DitResult<()> {
    match fs::remove_file(path) {
        Ok(()) => Ok(()),
        Err(e) if e.kind() == io::ErrorKind::NotFound => Ok(()),
        Err(_) => Err(FsError::FileRemoveError(path_to_string(path)).into()),
    }
}


/// Creates a file and all the necessary subdirectories (if they don't exist) and maps
/// the result to [`FsError`]
pub fn create_file_all(path: &Path) -> DitResult<()> {
    if let Some(parent) = Path::new(path).parent() {
        fs::create_dir_all(parent)
            .map_err(|_| FsError::DirCreateError(path_to_string(path)))?;
    }

    File::create(path)
        .map_err(|_| FsError::FileCreateError(path_to_string(path)))?;

    Ok(())
}
