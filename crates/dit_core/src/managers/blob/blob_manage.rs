use crate::managers::blob::BlobMgr;
use crate::helpers::{
    copy_file, copy_with_hash_as_name, remove_file_if_exists, rename_file
};
use crate::errors::DitResult;
use std::path::{Path, PathBuf};

/// API
impl BlobMgr {
    /// Creates a temporary blob which is placed in the stage.
    /// During the commit, this blob may be moved to the permanent blobs' location. \
    /// Returns the file hash
    pub fn create_temp_blob(&self, source: &Path) -> DitResult<String> {
        let dest = self.repo.stage();
        let hash = copy_with_hash_as_name(source, dest)?;
        Ok(hash)
    }

    /// Moves the file with an already known hash to the temporary blobs
    pub fn create_temp_blob_with_hash(&self, source_file: &Path, hash: String) -> DitResult<()> {
        let dest = self.repo.stage().join(&hash);
        copy_file(source_file, &dest)
    }

    /// Moves the temporary blob (creates with `create_stage_blob`) to the
    /// permanent blobs' location
    pub fn commit_temp_blob(&self, hash: String) -> DitResult<()> {
        let source = self.repo.stage().join(&hash);
        let target = self.repo.blobs().join(hash);

        rename_file(&source, &target)
    }

    /// Removes a staged (temporary blob)
    pub fn remove_temp_blob(&self, hash: String) -> DitResult<()> {
        let path = self.repo.stage().join(hash);
        remove_file_if_exists(&path)
    }

    /// Returns the path of a blob given its hash
    pub fn get_blob_path(&self, hash: String) -> PathBuf {
        self.repo.blobs().join(hash)
    }
}
