use crate::errors::{DitResult, FsError};
use crate::helpers::path_to_string;
use std::fs;
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;


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


/// Reads from a [`BufReader`] to a buffer and maps the error to [`FsError`]
///
/// Returns the number of bytes read
pub fn read_from_buf_reader(
    reader: &mut BufReader<File>,
    buffer: &mut [u8],
    file_path: &Path
) -> DitResult<usize>
{
    reader.read(buffer)
        .map_err(|_| FsError::FileReadError(path_to_string(file_path)).into())
}


/// Creates and returns a [`BufReader`] and maps the error to [`FsError`]
///
/// Note: returns an error if the file doesn't exist
pub fn get_buf_reader(path: &Path) -> DitResult<BufReader<File>> {
    File::open(path)
        .map(BufReader::new)
        .map_err(|_| FsError::FileOpenError(path_to_string(path)).into())
}
