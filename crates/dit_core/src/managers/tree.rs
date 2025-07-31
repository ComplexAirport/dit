//! This module manages the trees in dit version control system
//!
//! Trees are *snapshots* of all files included in the repo during a particular commit.
//! They store all files and their corresponding blob hashes. Each commit has one
//! corresponding tree.
//!
//! Let's say we do an initial commit with only one file, `a.txt`.
//! We can imagine the tree for this commit as something like this (simplified): \
//! `.dit/trees/18b7cb09..`
//! ```json
//! {
//!     "files": {
//!         "a.txt": "b1ac8a822.."
//!     }
//! }
//! ```
//!
//! Suppose in the next commit we commited a single new file, `b.txt` and did not change `a.txt`
//! The tree for this commit will look like this: \
//! `.dit/trees/c52b214f..`
//! ```json
//! {
//!     "files": {
//!         "a.txt": "b1ac8a82..",
//!         "b.txt": "273c662e.."
//!     }
//! }
//! ```

use crate::blob::BlobMgr;
use crate::repo::Repo;
use crate::stage::StagedFiles;
use crate::errors::{DitResult, TreeError};
use crate::helpers::{create_file_all, get_buf_writer, read_to_string, transfer_data, write_to_file};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::BTreeMap;
use std::path::PathBuf;
use std::rc::Rc;

/// Manages the trees in our Dit version control system
pub struct TreeMgr {
    repo: Rc<Repo>,
}

/// Constructors
impl TreeMgr {
    pub fn from(repo: Rc<Repo>) -> Self {
        Self { repo }
    }
}

/// API
impl TreeMgr {
    /// Creates a tree and returns the tree hash
    pub fn create_tree(&self,
                       staged_files: &StagedFiles,
                       parent_tree_hash: Option<String>,
                       blob_mgr: &mut BlobMgr)
        -> DitResult<String>
    {
        // we will operate on the collection of files sorted by their relative paths
        // this will prevent tree hash inconsistencies across systems and prevent the tree
        // hash being dependent on traversal order

        let mut files: BTreeMap<PathBuf, String> = if let Some(parent_tree) = parent_tree_hash {
            self.get_tree(parent_tree)?.files
        } else {
            BTreeMap::new()
        };

        for (relative_path, staged_path) in &staged_files.files {
            let blob_hash = blob_mgr.create_blob(staged_path)?;
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

    /// Reads and returns a tree from the tree's hash
    pub fn get_tree(&self, tree_hash: String) -> DitResult<Tree> {
        let path = self.repo.trees().join(tree_hash.clone());

        let serialized = read_to_string(&path)?;

        let tree: Tree = serde_json::from_str(&serialized)
            .map_err(|_| TreeError::DeserializationError(tree_hash))?;

        Ok(tree)
    }

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

            let abs_path = self.repo.get_absolute_path_unchecked(rel_path);
            create_file_all(&abs_path)?;
            let mut writer = get_buf_writer(&abs_path)?;

            transfer_data(&mut reader, &mut writer, &abs_path)?;
        }
        
        Ok(())
    }
}

/// Private helper methods
impl TreeMgr {
    /// Writes the tree to the trees directory
    fn write_tree(&self, tree: &Tree) -> DitResult<()> {
        let serialized = serde_json::to_string(&tree)
            .map_err(|_| TreeError::SerializationError(tree.hash.clone()))?;

        let path = self.repo.trees().join(tree.hash.clone());
        write_to_file(&path, &serialized)?;

        Ok(())
    }
}

/// Represents the tree object
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tree {
    /// Maps the relative file paths to corresponding blob hashes
    pub files: BTreeMap<PathBuf, String>,

    /// Represents the tree hash
    #[serde(skip)]
    pub hash: String,
}
