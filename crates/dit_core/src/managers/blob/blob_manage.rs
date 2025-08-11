use crate::managers::blob::BlobMgr;
use crate::helpers::{copy_file, copy_with_hash_as_name, get_buf_reader, remove_file, rename_file};
use crate::errors::DitResult;
use std::path::Path;

/// API
impl BlobMgr {
    /// Moves the file with an already known hash to the temporary blobs
    pub fn create_temp_blob_with_hash<P: AsRef<Path>>(&self, source_file: P, hash: String)
        -> DitResult<()>
    {
        let dest = self.repo.stage().join(&hash);
        copy_file(source_file, dest)?;
        Ok(())
    }

    /// Removes a staged (temporary blob)
    pub fn remove_temp_blob(&self, hash: String) -> DitResult<()> {
        let path = self.repo.stage().join(hash);
        if path.is_file() {
            remove_file(path)
        } else {
            Ok(())
        }
    }

    /// Creates a temporary blob which is placed in the stage.
    /// During the commit, this blob may be moved to the permanent blobs' location. \
    /// Returns the file hash
    pub fn create_temp_blob<P: AsRef<Path>>(&self, source_file: P) -> DitResult<String> {
        let source = get_buf_reader(source_file.as_ref())?;
        let dest = self.repo.stage();
        let hash = copy_with_hash_as_name(source, dest)?;
        Ok(hash)
    }

    /// Moves the temporary blob (creates with `create_stage_blob`) to the
    /// permanent blobs' location
    pub fn commit_temp_blob(&self, hash: String) -> DitResult<()> {
        let source = self.repo.stage().join(&hash);
        let target = self.repo.blobs().join(&hash);

        rename_file(source, target)?;

        Ok(())
    }
}
