use crate::helpers::{get_buf_reader, read_from_buf_reader, HASHING_BUFFER_SIZE};
use crate::errors::DitResult;
use std::io::Write;
use std::path::Path;
use blake3::Hasher;

/// Writer to a writer and calculates the hash
pub struct HashingWriter<W: Write> {
    inner: W,
    hasher: Hasher,
}

impl<W: Write> HashingWriter<W> {
    pub fn new(inner: W) -> Self {
        Self {
            inner,
            hasher: Hasher::new(),
        }
    }

    pub fn finalize(self) -> blake3::Hash {
        self.hasher.finalize()
    }

    pub fn finalize_string(self) -> String {
        format!("{}", self.hasher.finalize())
    }
}

impl<W: Write> Write for HashingWriter<W> {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let n = self.inner.write(buf)?;
        self.hasher.update(buf);
        Ok(n)
    }

    fn flush(&mut self) -> std::io::Result<()> { self.inner.flush() }
}

#[derive(Debug, Default)]
pub struct DitHasher {
    hasher: Hasher
}

impl DitHasher {
    pub fn new() -> Self {
        Self {
            hasher: Hasher::new()
        }
    }

    pub fn update(&mut self, buf: &[u8]) {
        self.hasher.update(buf);
    }

    pub fn finalize(self) -> blake3::Hash {
        self.hasher.finalize()
    }

    pub fn finalize_string(self) -> String {
        format!("{}", self.hasher.finalize())
    }
}


pub fn hash_file(path: &Path) -> DitResult<String> {
    let mut reader = get_buf_reader(path)?;
    let mut hasher = DitHasher::new();

    let mut buf = vec![0; HASHING_BUFFER_SIZE];
    loop {
        let n = read_from_buf_reader(&mut reader, &mut buf, path)?;
        if n == 0 {
            break;
        }
        hasher.update(&buf[..n]);
    }
    Ok(hasher.finalize_string())
}
