use crate::managers::blob::BlobMgr;
use crate::helpers::get_buf_reader;
use crate::errors::DitResult;
use std::io::BufReader;
use std::fs::File;


/// Getters
impl BlobMgr {
    /// Returns the blob content reader based on it's hash
    pub fn get_blob_reader<S: Into<String>>(&self, hash: S) -> DitResult<BufReader<File>> {
        let path = self.repo.blobs().join(hash.into());
        let reader = get_buf_reader(&path)?;
        Ok(reader)
    }
}