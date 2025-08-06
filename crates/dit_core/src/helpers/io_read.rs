use crate::errors::{DitResult, FsError};
use crate::helpers::{path_to_string, BUFFER_SIZE};
use std::fs;
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;
use sha2::{Digest, Sha256};

/// Reads a file using [`fs::read_to_string`] and maps the error to [`FsError`]
pub fn read_to_string<P: AsRef<Path>>(path: P) -> DitResult<String> {
    let path = path.as_ref();
    fs::read_to_string(path)
        .map_err(|_| FsError::FileReadError(path_to_string(path)).into())
}


/// Reads from a [`BufReader`] to a buffer and maps the error to [`FsError`]
///
/// Returns the number of bytes read
pub fn read_from_buf_reader<P: AsRef<Path>>(
    reader: &mut BufReader<File>,
    buffer: &mut [u8],
    file_path: P
) -> DitResult<usize>
{
    let file_path = file_path.as_ref();

    reader.read(buffer)
        .map_err(|_| FsError::FileReadError(path_to_string(file_path)).into())
}


/// Creates and returns a [`BufReader`] and maps the error to [`FsError`]
///
/// Note: returns an error if the file doesn't exist
pub fn get_buf_reader<P: AsRef<Path>>(path: P) -> DitResult<BufReader<File>> {
    let path = path.as_ref();
    File::open(path)
        .map(BufReader::new)
        .map_err(|_| FsError::FileOpenError(path_to_string(path)).into())
}


/// Calculates the hash of a file
pub fn calculate_hash<P: AsRef<Path>>(path: P) -> DitResult<String> {
    let path = path.as_ref();
    let mut reader = get_buf_reader(path)?;

    let mut buffer = [0; BUFFER_SIZE];
    let mut hasher = Sha256::new();
    loop {
        let n = read_from_buf_reader(
            &mut reader,
            &mut buffer,
            path
                .file_name()
                .unwrap_or_default())?;

        if n == 0 {
            break;
        }
        hasher.update(&buffer[..n]);
    }
    let hash = format!("{:x}", hasher.finalize());
    Ok(hash)
}
