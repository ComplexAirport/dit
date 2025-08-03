use crate::managers::blob::BlobMgr;
use crate::helpers::{copy_with_hash_as_name, get_buf_reader};
use crate::errors::DitResult;
use std::path::PathBuf;

/// API
impl BlobMgr {
    /// Adds a target file to the blobs and returns the hash
    pub fn create_blob<P: Into<PathBuf>>(&self, source_file: P) -> DitResult<String> {
        let source_file = source_file.into();
        let reader = get_buf_reader(&source_file)?;

        let hash = copy_with_hash_as_name(reader, self.repo.blobs())?;

        Ok(hash)
    }
}
