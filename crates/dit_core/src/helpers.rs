use crate::errors::{DitResult, FsError};
use std::fs;
use std::fs::File;
use std::io::{BufReader, BufWriter, Read, Write};
use std::path::{Path, PathBuf};


/// Reads a file using [`fs::read_to_string`] and maps the error to [`FsError`]
pub fn read_to_string<P: AsRef<Path>>(path: P) -> DitResult<String> {
    let path = path.as_ref();
    fs::read_to_string(path)
        .map_err(|_| FsError::FileReadError(path.display().to_string()).into())
}


/// Writes to a file using [`fs::write`] and maps the error to [`FsError`]
pub fn write_to_file<P: AsRef<Path>, S: AsRef<str>>(path: P, content: S) -> DitResult<()> {
    let path = path.as_ref();
    fs::write(path, content.as_ref())
        .map_err(|_| FsError::FileWriteError(path.display().to_string()).into())
}


/// Removes a file using [`fs::remove_file`] and maps the error to [`FsError`]
pub fn remove_file<P: AsRef<Path>>(path: P) -> DitResult<()> {
    let path = path.as_ref();
    fs::remove_file(path)
        .map_err(|_| FsError::FileRemoveError(path.display().to_string()).into())
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


/// Creates and returns a [`BufWriter`] and maps the error to [`FsError`].
///
/// Note: creates the file if it doesn't exist and overrides it if it does
pub fn get_buf_writer<P: AsRef<Path>>(path: P) -> DitResult<BufWriter<File>> {
    let path = path.as_ref();
    File::create(path)
        .map(BufWriter::new)
        .map_err(|_| FsError::FileOpenError(path.display().to_string()).into())
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


/// Writes to a [`BufWriter`] from a buffer and maps the error to [`FsError`]
pub fn write_to_buf_writer<P: AsRef<Path>>(
    writer: &mut BufWriter<File>,
    buffer: &[u8],
    file_path: P
) -> DitResult<()>
{
    let file_path = file_path.as_ref();

    writer.write_all(buffer)
        .map_err(|_| FsError::FileWriteError(file_path.display().to_string()).into())
}


/// Resolves a given path to an absolute, canonical path.
///
/// - Follows symbolic links.
/// - Returns an error if the path does not exist.
/// - On Windows, strips extended-length path prefix (e.g. `\\?\C:\...`)
pub fn resolve_absolute_path(input: &Path) -> DitResult<PathBuf> {
    let canonical = fs::canonicalize(input)
        .map_err(|_| FsError::AbsPathResolveError(input.display().to_string()))?;

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
