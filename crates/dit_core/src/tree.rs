﻿//! This module manages the trees in dit version control system
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
use crate::dit_project::DitProject;
use crate::stage::StagedFiles;
use crate::errors::{DitResult, TreeError};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::BTreeMap;
use std::path::PathBuf;
use std::rc::Rc;
use crate::helpers::{read_to_string, write_to_file};

/// Manages the trees in our Dit version control system
pub struct TreeMgr {
    project: Rc<DitProject>,

    /// Represents the blobs manager [`BlobMgr`]
    blob_mgr: BlobMgr,
}

/// Constructors
impl TreeMgr {
    pub fn from(project: Rc<DitProject>) -> Self {
        let blob_mgr = BlobMgr::from(project.clone());
        Self { project, blob_mgr }
    }
}

/// API
impl TreeMgr {
    /// Creates a tree and returns the tree hash
    pub fn create_tree(&self, staged_files: &StagedFiles, parent_tree_hash: Option<String>)
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
            let blob_hash = self.blob_mgr.create_blob(staged_path)?;
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

/// Private helper methods
impl TreeMgr {
    /// Writes the tree to the trees directory
    fn write_tree(&self, tree: &Tree) -> DitResult<()> {
        let serialized = serde_json::to_string(&tree)
            .map_err(|_| TreeError::SerializationError(tree.hash.clone()))?;

        let path = self.project.trees().join(tree.hash.clone());
        write_to_file(&path, &serialized)?;

        Ok(())
    }

    /// Reads and returns a tree from the tree's hash
    fn get_tree(&self, tree_hash: String) -> DitResult<Tree> {
        let path = self.project.trees().join(tree_hash.clone());

        let serialized = read_to_string(&path)?;

        let tree: Tree = serde_json::from_str(&serialized)
            .map_err(|_| TreeError::DeserializationError(tree_hash))?;

        Ok(tree)
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
