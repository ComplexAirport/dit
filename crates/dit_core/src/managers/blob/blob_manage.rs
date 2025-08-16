use std::fs::File;
use crate::managers::blob::BlobMgr;
use crate::helpers::{
    compress_file, compress_file_hashed,
    create_temp_file, decompress_file,
    remove_file_if_exists, rename_file
};
use crate::errors::DitResult;
use std::path::{Path, PathBuf};

/// API
impl BlobMgr {
    /// Creates a blob. Returns its hash
    pub fn create_blob(&self, source: &Path) -> DitResult<String> {
        let (_, temp_file_path) = create_temp_file(self.repo.blobs())?;
        let hash = compress_file_hashed(source, &temp_file_path)?;
        let dest = self.get_blob_path(hash.clone());
        rename_file(&temp_file_path, &dest)?;
        Ok(hash)
    }

    /// Creates a blob with already pre-calculated hash
    pub fn create_blob_with_hash(&self, source: &Path, hash: String) -> DitResult<()> {
        let dest = self.get_blob_path(hash);
        compress_file(source, &dest)
    }

    /// Recovers the blob to the target file
    pub fn recover_blob(&self, hash: String, target: &Path) -> DitResult<()> {
        decompress_file(&self.get_blob_path(hash), target)
    }

    /// Removes a blob
    pub fn remove_blob(&self, hash: String) -> DitResult<()> {
        remove_file_if_exists(&self.get_blob_path(hash))
    }

    /// Returns the path of a blob given its hash
    pub fn get_blob_path(&self, hash: String) -> PathBuf {
        self.repo.blobs().join(hash)
    }
}
