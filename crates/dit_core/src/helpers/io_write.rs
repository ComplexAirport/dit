//! Superfast functions for IO writing operations

use crate::helpers::{path_to_string, rename_file, BUFFER_SIZE};
use crate::helpers::io_read::read_from_buf_reader;
use crate::helpers::temp_file::create_temp_file;
use crate::helpers::hashing::HashingWriter;
use crate::errors::{DitResult, FsError, OtherError};
use std::fs;
use std::fs::File;
use std::io::{self, BufReader, BufWriter, Write};
use std::path::Path;

/// Writes to a file using [`fs::write`] and maps the error to [`FsError`]
pub fn write_to_file<P, S>(path: P, content: S) -> DitResult<()>
where
    P: AsRef<Path>,
    S: AsRef<str>
{
    fs::write(&path, content.as_ref())
        .map_err(|_| FsError::FileWriteError(path_to_string(path)).into())
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
        .map_err(|_| FsError::FileWriteError(path_to_string(file_path)).into())
}



/// Copies the content of a given file to the given destination
pub fn copy_file(src: &Path, dest: &Path) -> DitResult<()>
{
    fs::copy(&src, &dest)
        .map_err(|_| FsError::FileCopyError(
            path_to_string(src),
            path_to_string(dest)
        ))?;
    Ok(())
}


/// Reads data from [`BufReader`] and writes to a [`BufWriter`] mapping the error to
/// [`FsError`]
pub fn copy_file_buffered(
    reader: &mut BufReader<File>,
    writer: &mut BufWriter<File>,
    filename: &Path,
) -> DitResult<()>
{
    let mut buffer = [0; BUFFER_SIZE];
    loop {
        let n = read_from_buf_reader(reader, &mut buffer, filename)?;
        if n == 0 {
            break;
        }
        write_to_buf_writer(writer, &buffer[..n], filename)?;
    }

    Ok(())
}


/// Reads data from [`BufReader`] and writes to a [`BufWriter`] mapping the error to
/// [`FsError`] while also calculating the content hash. Returns the hash.
pub fn copy_file_hashed(src: &Path, dest_path: &Path, dest_file: File) -> DitResult<String>
{
    let src_file = File::open(src)
        .map_err(|_| FsError::FileOpenError(path_to_string(src)))?;

    let mut reader = BufReader::with_capacity(BUFFER_SIZE, src_file);

    let writer = BufWriter::with_capacity(BUFFER_SIZE, dest_file);
    let mut hasher = HashingWriter::new(writer);

    io::copy(&mut reader, &mut hasher)
        .map_err(|_| FsError::FileCopyError(
            path_to_string(src),
            path_to_string(dest_path)
        ))?;

    hasher.flush()
        .map_err(|_| OtherError::BufferFlushError)?;

    Ok(hasher.finalize_string())
}


/// Copies a file to a new destination and sets the new file's name
/// as its hash. Returns the hash.
pub fn copy_with_hash_as_name(src: &Path, dest_path: &Path) -> DitResult<String>
{
    let (temp_file, temp_file_path) = create_temp_file(dest_path)?;
    let hash = copy_file_hashed(src, &temp_file_path, temp_file)?;
    let dest = dest_path.join(&hash);

    rename_file(&temp_file_path, &dest)?;

    Ok(hash)
}



/// Creates and returns a [`BufWriter`] given a target path and
/// maps the error to [`FsError`].
///
/// Note: creates the file if it doesn't exist and overrides it if it does
pub fn get_buf_writer(path: &Path) -> DitResult<BufWriter<File>> {
    File::create(path)
        .map(BufWriter::new)
        .map_err(|_| FsError::FileOpenError(path_to_string(path)).into())
}
