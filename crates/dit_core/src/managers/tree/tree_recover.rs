use crate::errors::DitResult;
use crate::helpers::create_file_all;
use crate::managers::blob::BlobMgr;
use crate::managers::tree::TreeMgr;
use rayon::prelude::*;

impl TreeMgr {
    /// Recovers a tree given a [`Tree`] (writes all files to the project root)
    ///
    /// Note: files not included in the [`Tree`] will remain unchanged
    pub fn recover_tree(
        &self,
        tree_hash: String,
        blob_mgr: &mut BlobMgr
    ) -> DitResult<()>
    {
        let index = self.get_tree(tree_hash)?.index;

        index.files
            .into_par_iter()
            .try_for_each(|(rel_path, entry)| {
                let abs_path = self.repo.abs_path_from_repo(&rel_path, true)?;
                create_file_all(&abs_path)?;
                blob_mgr.recover_blob(entry.hash, &rel_path)
            })
    }
}