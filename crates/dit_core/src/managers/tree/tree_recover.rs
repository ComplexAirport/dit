use crate::errors::DitResult;
use crate::helpers::{create_file_all, copy_file};
use crate::managers::blob::BlobMgr;
use crate::managers::tree::TreeMgr;

impl TreeMgr {
    /// Recovers a tree given a [`Tree`] (writes all files to the project root)
    ///
    /// Note: files not included in the [`Tree`] will remain unchanged
    pub fn recover_tree(
        &self,
        tree_hash: String,
        blob_mgr: &mut BlobMgr)
        -> DitResult<()>
    {
        let tree = self.get_tree(tree_hash)?;
        let files = tree.files;

        for (rel_path, blob_hash) in files {
            let abs_path = self.repo.abs_path_from_repo(&rel_path, true)?;
            create_file_all(&abs_path)?;
            copy_file(&abs_path, &blob_mgr.get_blob_path(blob_hash))?;
        }

        Ok(())
    }
}