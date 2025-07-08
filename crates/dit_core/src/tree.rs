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

use std::path::{Path, PathBuf};
use std::collections::BTreeMap;
use std::io::{self, Error};
use sha2::{Sha256, Digest};
use serde::{Serialize, Deserialize};
use serde_json;
use crate::blob::BlobMgr;
use crate::constants::TREES_ROOT;

/// This struct is a tree builder that represents staged files, which may later be used
/// in [`TreeMgr`] to create it
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StagedFiles {
    pub files: Vec<PathBuf>,
}

impl StagedFiles {
    pub fn new() -> Self {
        Self {
            files: Vec::new(),
        }
    }

    pub fn from(files: Vec<PathBuf>) -> Self {
        Self {
            files,
        }
    }
}

impl StagedFiles {
    /// Add a file to the tree builder given its path. \
    /// This method does not create any blobs or calculate hashes
    pub fn add_file<P: Into<PathBuf>>(&mut self, path: P) -> io::Result<()> {
        let path = path.into();

        if !path.is_file() {
            return Err(Error::new(io::ErrorKind::InvalidInput,
                                      "the specified path is not a file"));
        }

        self.files.push(path);

        Ok(())
    }

    pub fn remove_file<P: Into<PathBuf>>(&mut self, path: P) {
        let path = path.into();
        if let Some(pos) = self.files.iter().position(|p| *p == path) {
            self.files.remove(pos);
        }
    }
}


/// Manages the trees in our Dit version control system
pub struct TreeMgr {
    /// Represents the trees directory, [`TREES_ROOT`]
    root_path: PathBuf,

    /// Represents the project directory, where the `.dit` is located
    project_path: PathBuf,

    /// Represents the blobs manager [`BlobMgr`]
    blob_mgr: BlobMgr,
}

/// Constructors
impl TreeMgr {
    /// Constructs the object given the project path (inside which the `.dit` is located)
    /// and a blob manager
    pub fn from<P: Into<PathBuf>>(project_path: P, blob_mgr: BlobMgr) -> io::Result<Self> {
        let project_path = project_path.into();
        if !project_path.is_dir() {
            panic!(
                "the specified path {} is not a directory",
                project_path.display()
            )
        }

        let root = project_path.join(TREES_ROOT);
        if !root.is_dir() {
            std::fs::create_dir_all(&root)?;
        }

        Ok(Self {
            root_path: root,
            project_path,
            blob_mgr,
        })
    }

    /// Constructs the object given the project path (inside which the `.dit` is located) \
    /// Creates a blob manager
    pub fn from_project<P: Into<PathBuf>>(project_path: P) -> io::Result<Self> {
        let project_path = project_path.into();
        let blob_mgr = BlobMgr::from_project(&project_path)?;
        Self::from(project_path, blob_mgr)
    }
}

/// API
impl TreeMgr {
    /// Creates a tree and returns the tree hash
    pub fn create_tree(&self, tree: &StagedFiles) -> io::Result<String> {
        // we will operate on the collection of files sorted by their relative paths
        // this will prevent tree hash inconsistencies across systems and prevent the tree
        // hash being dependent on traversal order
        let mut files: BTreeMap<PathBuf, String> = BTreeMap::new();

        for path in &tree.files {
            let blob_hash = self.blob_mgr.create_blob(path)?;
            let relative_path = self.get_relative_path(path)?;
            files.insert(relative_path, blob_hash);
        }

        let mut hasher = Sha256::new();
        for (path, blob_hash) in &files {
            hasher.update(path.to_string_lossy().as_bytes());
            hasher.update(blob_hash);
        }
        let hash = format!("{:x}", hasher.finalize());

        let tree = Tree { files, hash: hash.clone() };
        self.write_tree(&tree)?;

        Ok(hash)
    }

    /// Reads and returns a tree from the tree's hash
    pub fn get_tree(&self, tree_hash: String) -> io::Result<Tree> {
        let path = self.root_path.join(tree_hash.clone());
        let serialized = std::fs::read_to_string(path)?;
        let tree: Tree = serde_json::from_str(&serialized)?;
        Ok(tree)
    }
}

/// Private helper methods
impl TreeMgr {
    /// Writes the tree to the trees directory
    fn write_tree(&self, tree: &Tree) -> io::Result<()> {
        let serialized = serde_json::to_string_pretty(&tree)?;
        let path = self.root_path.join(tree.hash.clone());
        if !path.is_file() {
            std::fs::write(path, serialized)?;
        }
        Ok(())
    }

    /// Returns the path of a given path relative to the project path. \
    /// For example, if the project path is `D:\Projects\project1\` and the given path is
    /// `D:\Projects\project1\src\main.py`, this method will return `src\main.py`
    fn get_relative_path(&self, path: &Path) -> Result<PathBuf, Error> {
        match path.strip_prefix(&self.project_path) {
            Err(_) => Err(Error::new(
                io::ErrorKind::InvalidInput,
                format!("the path '{}' is not a inside the dit project", path.display())
            )),

            Ok(path) => Ok(path.to_path_buf()),
        }
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
