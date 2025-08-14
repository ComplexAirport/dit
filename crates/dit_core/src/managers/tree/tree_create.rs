use crate::managers::blob::BlobMgr;
use crate::managers::tree::TreeMgr;
use crate::models::{ChangeType, ModifiedFile, NewFile, Stage, Tree};
use crate::errors::DitResult;
use std::collections::BTreeMap;
use crate::helpers::DitHasher;

impl TreeMgr {
    /// Creates a tree from a stage and returns the tree hash
    pub fn create_tree(
        &self,
        stage: &Stage,
        parent_tree_hash: Option<String>,
        blob_mgr: &BlobMgr
    ) -> DitResult<String>
    {
        let mut files = if let Some(parent_tree) = parent_tree_hash {
            self.get_tree(parent_tree)?.files
        } else {
            BTreeMap::new()
        };

        let mut hasher = DitHasher::new();
        for (rel_path, change) in &stage.files {
            match change {
                ChangeType::New(NewFile { hash }) => {
                    blob_mgr.commit_temp_blob(hash.clone())?;
                    hasher.update(rel_path.to_string_lossy().as_bytes());
                    hasher.update(hash.as_bytes());
                    files.insert(rel_path.clone(), hash.clone());
                }

                ChangeType::Modified(ModifiedFile { new_hash, ..}) => {
                    blob_mgr.commit_temp_blob(new_hash.clone())?;
                    hasher.update(rel_path.to_string_lossy().as_bytes());
                    hasher.update(new_hash.as_bytes());
                    files.insert(rel_path.clone(), new_hash.clone());
                }

                ChangeType::Deleted => {
                    files.remove(rel_path);
                },

                _ => {}
            }
        }

        let hash = hasher.finalize_string();

        let tree = Tree {
            files,
            hash: hash.clone(),
        };

        self.write_tree(&tree)?;

        Ok(hash)
    }
}
