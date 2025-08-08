use crate::errors::DitResult;
use crate::helpers::{create_file_all, get_buf_writer, transfer_data};
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
            let mut reader = blob_mgr.get_blob_reader(blob_hash)?;

            let abs_path = self.repo.abs_path_from_repo(rel_path, true)?;
            create_file_all(&abs_path)?;
            let mut writer = get_buf_writer(&abs_path)?;

            transfer_data(&mut reader, &mut writer, &abs_path)?;
        }

        Ok(())
    }
}