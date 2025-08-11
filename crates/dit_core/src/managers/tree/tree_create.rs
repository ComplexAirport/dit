use crate::managers::blob::BlobMgr;
use crate::managers::tree::TreeMgr;
use crate::models::{ChangeType, ModifiedFile, NewFile, Stage, Tree};
use crate::errors::DitResult;
use std::collections::BTreeMap;
use sha2::{Digest, Sha256};

impl TreeMgr {
    /// Creates a tree from a stage and returns the tree hash
    pub fn create_tree(
        &self,
        stage: &Stage,
        parent_tree_hash: Option<String>,
        blob_mgr: &BlobMgr,
    ) -> DitResult<String>
    {
        // we will operate on the collection of files sorted by their relative paths
        // this will prevent tree hash inconsistencies across systems and prevent the tree
        // hash being dependent on traversal order

        let mut files = if let Some(parent_tree) = parent_tree_hash {
            self.get_tree(parent_tree)?.files
        } else {
            BTreeMap::new()
        };

        for (rel_path, change) in &stage.files {
            match change {
                ChangeType::New(NewFile { hash }) => {
                    blob_mgr.commit_temp_blob(hash.clone())?;
                    files.insert(rel_path.clone(), hash.clone());
                }

                ChangeType::Modified(ModifiedFile { new_hash, ..}) => {
                    blob_mgr.commit_temp_blob(new_hash.clone())?;
                    files.insert(rel_path.clone(), new_hash.clone());
                }

                ChangeType::Deleted => {
                    files.remove(rel_path);
                },

                _ => {}
            }
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
