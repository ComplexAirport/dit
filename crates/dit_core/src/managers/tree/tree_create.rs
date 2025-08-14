use crate::managers::blob::BlobMgr;
use crate::managers::tree::TreeMgr;
use crate::models::{ChangeType, ModifiedFile, NewFile, Stage, Tree};
use crate::errors::DitResult;
use crate::helpers::DitHasher;
use rayon::prelude::*;
use std::collections::{BTreeMap, HashSet};

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

        let mut blobs_to_commit = HashSet::new();
        for (rel_path, change) in &stage.files {
            match change {
                ChangeType::New(NewFile { hash }) => {
                    blobs_to_commit.insert(hash.clone());
                    files.insert(rel_path.clone(), hash.clone());
                }

                ChangeType::Modified(ModifiedFile { new_hash, ..}) => {
                    blobs_to_commit.insert(new_hash.clone());
                    files.insert(rel_path.clone(), new_hash.clone());
                }

                ChangeType::Deleted => {
                    files.remove(rel_path);
                },

                _ => {}
            }
        }

        // Calculate the hash in deterministic order
        let mut hasher = DitHasher::new();
        for (rel_path, blob_hash) in &files {
            hasher.update(rel_path.to_string_lossy().as_bytes());
            hasher.update(blob_hash.as_bytes());
        }
        let hash = hasher.finalize_string();

        // Commit the temporary blobs in parallel
        blobs_to_commit.into_par_iter()
            .try_for_each(|x| blob_mgr.commit_temp_blob(x))?;

        let tree = Tree {
            files,
            hash: hash.clone(),
        };

        self.write_tree(&tree)?;

        Ok(hash)
    }
}
