use crate::errors::{DitResult, FsError};
use std::fs;
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;


/// Reads a file using [`fs::read_to_string`] and maps the error to [`FsError`]
pub fn read_to_string<P: AsRef<Path>>(path: P) -> DitResult<String> {
    let path = path.as_ref();
    fs::read_to_string(path)
        .map_err(|_| FsError::FileReadError(path.display().to_string()).into())
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
        .map_err(|_| FsError::FileReadError(file_path.display().to_string()).into())
}


/// Creates and returns a [`BufReader`] and maps the error to [`FsError`]
///
/// Note: returns an error if the file doesn't exist
pub fn get_buf_reader<P: AsRef<Path>>(path: P) -> DitResult<BufReader<File>> {
    let path = path.as_ref();
    File::open(path)
        .map(BufReader::new)
        .map_err(|_| FsError::FileOpenError(path.display().to_string()).into())
}
