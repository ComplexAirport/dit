use crate::errors::DitResult;
use crate::helpers::{get_buf_reader_with_cap, get_buf_writer_with_cap, DitHasher, ZSTD_BUFFER_SIZE, ZSTD_COMPRESSION_LEVEL};
use zstd::stream::{write::Encoder, read::Decoder};
use std::io::{self, Read, Write};
use std::path::Path;


/// Compresses a file using the ZSTD algorithm
pub fn compress_file(src: &Path, dest: &Path) -> DitResult<()> {
    let mut reader = get_buf_reader_with_cap(ZSTD_BUFFER_SIZE, src)?;
    let writer = get_buf_writer_with_cap(ZSTD_BUFFER_SIZE, dest)?;
    let mut encoder = Encoder::new(writer, ZSTD_COMPRESSION_LEVEL as i32)?;

    io::copy(&mut reader, &mut encoder)?;

    let mut writer = encoder.finish()?;
    writer.flush()?;

    Ok(())
}

/// Compresses a file using the ZSTD algorithm and calculates its hash
pub fn compress_file_hashed(src: &Path, dest: &Path) -> DitResult<String> {
    let mut reader = get_buf_reader_with_cap(ZSTD_BUFFER_SIZE, src)?;
    let writer = get_buf_writer_with_cap(ZSTD_BUFFER_SIZE, dest)?;
    let mut encoder = Encoder::new(writer, ZSTD_COMPRESSION_LEVEL as i32)?;

    let mut hasher = DitHasher::new();
    let mut buf = vec![0; ZSTD_BUFFER_SIZE];
    loop {
        let n = reader.read(&mut buf)?;
        if n == 0 {
            break;
        }

        hasher.update(&buf[..n]);
        encoder.write_all(&buf[..n])?;
    }

    let mut writer = encoder.finish()?;
    writer.flush()?;

    Ok(hasher.finalize_string())
}


/// Decompresses a file using ZSTD algorithm
pub fn decompress_file(src: &Path, dest: &Path) -> DitResult<()> {
    let reader = get_buf_reader_with_cap(ZSTD_BUFFER_SIZE, src)?;
    let mut decoder = Decoder::new(reader)?;
    let mut writer = get_buf_writer_with_cap(ZSTD_BUFFER_SIZE, dest)?;
    io::copy(&mut decoder, &mut writer)?;
    writer.flush()?;
    Ok(())
}
