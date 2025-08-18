use crate::helpers::{DitHasher, ZSTD_BUFFER_SIZE, ZSTD_COMPRESSION_LEVEL};
use crate::errors::DitResult;
use std::io::{self, BufReader, BufWriter, Read, Write};
use zstd::stream::{write::Encoder, read::Decoder};
use std::fs::File;
use std::path::Path;


/// Compresses a file using the ZSTD algorithm
pub fn compress_file(src: &Path, dest: &Path) -> DitResult<()> {
    let mut reader = BufReader::with_capacity(ZSTD_BUFFER_SIZE, File::open(src)?);
    let writer = BufWriter::with_capacity(ZSTD_BUFFER_SIZE, File::create(dest)?);
    let mut encoder = Encoder::new(writer, ZSTD_COMPRESSION_LEVEL as i32)?;

    io::copy(&mut reader, &mut encoder)?;

    let mut writer = encoder.finish()?;
    writer.flush()?;

    Ok(())
}

/// Compresses a file using the ZSTD algorithm and calculates its hash
pub fn compress_file_hashed(src: &Path, dest: &Path) -> DitResult<String> {
    let mut reader = BufReader::with_capacity(ZSTD_BUFFER_SIZE, File::open(src)?);
    let writer = BufWriter::with_capacity(ZSTD_BUFFER_SIZE, File::create(dest)?);
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
    let reader = BufReader::with_capacity(ZSTD_BUFFER_SIZE, File::open(src)?);
    let mut decoder = Decoder::new(reader)?;
    let mut writer = BufWriter::with_capacity(ZSTD_BUFFER_SIZE, File::create(dest)?);
    io::copy(&mut decoder, &mut writer)?;
    writer.flush()?;
    Ok(())
}

