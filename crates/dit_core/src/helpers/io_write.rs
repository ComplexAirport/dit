use crate::constants::BUFFER_SIZE;
use crate::errors::{DitResult, FsError};
use crate::helpers::io_read::read_from_buf_reader;
use std::fs;
use std::fs::File;
use std::io::{BufReader, BufWriter, Write};
use std::path::Path;
use sha2::{Digest, Sha256};


/// Writes to a file using [`fs::write`] and maps the error to [`FsError`]
pub fn write_to_file<P: AsRef<Path>, S: AsRef<str>>(path: P, content: S) -> DitResult<()> {
    let path = path.as_ref();
    fs::write(path, content.as_ref())
        .map_err(|_| FsError::FileWriteError(path.display().to_string()).into())
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


/// Reads data from [`BufReader`] and writes to a [`BufWriter`] mapping the error to
/// [`FsError`]
pub fn transfer_data<P: AsRef<Path>>(
    reader: &mut BufReader<File>,
    writer: &mut BufWriter<File>,
    filename: P,
) -> DitResult<()>
{
    let filename = filename.as_ref();
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
pub fn transfer_data_hashed<P: AsRef<Path>>(
    reader: &mut BufReader<File>,
    writer: &mut BufWriter<File>,
    filename: P,
) -> DitResult<String>
{
    let filename = filename.as_ref();
    let mut buffer = [0; BUFFER_SIZE];
    let mut hasher = Sha256::new();
    loop {
        let n = read_from_buf_reader(reader, &mut buffer, filename)?;
        if n == 0 {
            break;
        }
        hasher.update(&buffer[..n]);
        write_to_buf_writer(writer, &buffer[..n], filename)?;
    }
    let hash = format!("{:x}", hasher.finalize());

    Ok(hash)
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

