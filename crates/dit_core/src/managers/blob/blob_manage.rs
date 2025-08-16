use crate::managers::blob::BlobMgr;
use crate::helpers::{
    copy_file, copy_with_hash_as_name, remove_file_if_exists,
};
use crate::errors::DitResult;
use std::path::{Path, PathBuf};

/// API
impl BlobMgr {
    /// Creates a blob. Returns its hash
    pub fn create_blob(&self, source: &Path) -> DitResult<String> {
        copy_with_hash_as_name(source, self.repo.blobs())
    }

    /// Creates a blob with already pre-calculated hash
    pub fn create_blob_with_hash(&self, source: &Path, hash: String) -> DitResult<()> {
        copy_file(source, &self.get_blob_path(hash))
    }

    /// Removes a blob
    pub fn remove_blob(&self, hash: String) -> DitResult<()> {
        remove_file_if_exists(&self.get_blob_path(hash))
    }

    /// Returns the path of a blob given its hash
    pub fn get_blob_path(&self, hash: String) -> PathBuf {
        self.repo.blobs().join(hash)
    }

    /// Recovers the blob to the target file
    pub fn recover_blob(&self, hash: String, target: &Path) -> DitResult<()> {
        copy_file(&self.get_blob_path(hash), target)
    }
}
