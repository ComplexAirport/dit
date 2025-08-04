use crate::managers::tree::TreeMgr;
use crate::models::{Stage, Tree};
use crate::helpers::rename_file;
use crate::errors::DitResult;
use std::collections::BTreeMap;
use std::path::PathBuf;
use sha2::{Digest, Sha256};

impl TreeMgr {
    /// Creates a tree from a stage and returns the tree hash
    pub fn create_tree(
        &self,
        stage: &Stage,
        parent_tree_hash: Option<String>
    ) -> DitResult<String>
    {
        // we will operate on the collection of files sorted by their relative paths
        // this will prevent tree hash inconsistencies across systems and prevent the tree
        // hash being dependent on traversal order

        let mut files: BTreeMap<PathBuf, String> = if let Some(parent_tree) = parent_tree_hash {
            self.get_tree(parent_tree)?.files
        } else {
            BTreeMap::new()
        };

        for (relative_path, temp_blob_path) in &stage.files {
            let blob_hash = temp_blob_path.file_name()
                .unwrap()
                .to_string_lossy()
                .to_string();

            let target_file = self.repo.blobs().join(&blob_hash);
            rename_file(temp_blob_path, &target_file)?;
            files.insert(relative_path.clone(), blob_hash);
        }

        let mut hasher = Sha256::new();
        for (path, blob_hash) in &files {
            hasher.update(path.to_string_lossy().as_bytes());
            hasher.update(blob_hash);
        }
        let hash = format!("{:x}", hasher.finalize());

        let tree = Tree {
            files,
            hash: hash.clone(),
        };
        self.write_tree(&tree)?;

        Ok(hash)
    }
}
