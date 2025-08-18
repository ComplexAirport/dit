use crate::errors::{DitResult, FsError};
use crate::helpers::path_to_string;
use std::path::Path;
use std::fs;


/// Reads a file using [`fs::read_to_string`] and maps the error to [`FsError`]
pub fn read_to_string(path: &Path) -> DitResult<String> {
    let mut s = fs::read_to_string(path)
        .map_err(|_| FsError::FileReadError(path_to_string(path)))?;

    // Strip the BOM if it exists
    if s.starts_with('\u{feff}') {
        s = s.trim_start_matches('\u{feff}').to_string();
    }

    Ok(s)
}
