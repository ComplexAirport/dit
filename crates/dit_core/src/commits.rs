use crate::constants::{COMMITS_ROOT, STAGED_FILE, HEAD_FILE};
use crate::trees::{TreeBuilder, TreeMgr};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::io;
use std::path::{Path, PathBuf};
use std::time::SystemTime;

/// Manages the commits in our Dit version control system
pub struct CommitMgr {
    /// Represents the commits directory, [`COMMITS_ROOT`]
    root_path: PathBuf,

    /// Represents the staged file, [`STAGED_FILE`]
    staged_file: PathBuf,

    /// Represents the head file, [`HEAD_FILE`]
    head_file: PathBuf,

    /// Represents the tree manager [`TreeMgr`]
    tree_mgr: TreeMgr,
}

/// Constructors
impl CommitMgr {
    /// Constructs the object given the project path (inside which the `.dit` is located)
    /// and a tree manager
    pub fn from<P: Into<PathBuf>>(project_path: P, tree_mgr: TreeMgr) -> io::Result<Self> {
        let project_path = project_path.into();
        if !project_path.is_dir() {
            panic!(
                "the specified path {} is not a directory",
                project_path.display()
            )
        }

        let root = project_path.join(COMMITS_ROOT);
        let staged_file = project_path.join(STAGED_FILE);
        let head_file = project_path.join(HEAD_FILE);
        if !root.is_dir() {
            std::fs::create_dir_all(&root)?;
        }
        if !staged_file.exists() {
            std::fs::write(&staged_file, "")?;
        }
        if !head_file.exists() {
            std::fs::write(&head_file, "")?;
        }

        Ok(Self {
            root_path: root,
            staged_file,
            head_file,
            tree_mgr,
        })
    }

    /// Constructs the object given the project path (inside which the `.dit` is located) \
    /// Creates a tree and blob managers
    pub fn from_project<P: Into<PathBuf>>(project_path: P) -> io::Result<Self> {
        let project_path = project_path.into();
        let tree_mgr = TreeMgr::from_project(&project_path)?;
        Self::from(project_path, tree_mgr)
    }
}

/// Manage commits
impl CommitMgr {
    /// Creates a commit and returns the commit hash
    pub fn create_commit(
        &self,
        author: String,
        message: String,
        tree: TreeBuilder,
        parent_commit_hash: Option<String>,
    ) -> io::Result<String> {
        let tree_hash = self.tree_mgr.create_tree(&tree)?;

        let timestamp = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs();

        let mut hasher = Sha256::new();
        hasher.update(&author);
        hasher.update(&message);
        hasher.update(timestamp.to_le_bytes());
        hasher.update(&tree_hash);
        hasher.update(parent_commit_hash.clone().unwrap_or(String::from('\0')));
        let commit_hash = format!("{:x}", hasher.finalize());

        let commit = Commit {
            author,
            message,
            timestamp,
            tree: tree_hash,
            parent: parent_commit_hash,
            hash: commit_hash.clone(),
        };

        self.write_commit(&commit)?;

        Ok(commit_hash)
    }

    /// Reads and returns a commit from the commit's hash
    fn get_commit(&self, hash: &str) -> io::Result<Commit> {
        let path = self.root_path.join(hash);
        let serialized = std::fs::read_to_string(path)?;
        let commit: Commit = serde_json::from_str(&serialized)?;
        Ok(commit)
    }
}

/// Manage staged files and head
impl CommitMgr {
    pub fn register_staged_files(&self, files: Vec<PathBuf>) -> io::Result<()> {
        let staged_files = StagedFiles { files };
        let serialized = serde_json::to_string(&staged_files)?;
        std::fs::write(&self.staged_file, serialized)?;
        Ok(())
    }

    pub fn register_head(&self, hash: String) -> io::Result<()> {
        std::fs::write(&self.head_file, hash)?;
        Ok(())
    }
}

/// Private helper methods
impl CommitMgr {
    /// Writes the commit to the commits directory
    fn write_commit(&self, commit: &Commit) -> io::Result<()> {
        let serialized = serde_json::to_string(&commit)?;
        let path = self.root_path.join(&commit.hash);
        std::fs::write(path, serialized)?;
        Ok(())
    }
}


/// Represents the commit object
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Commit {
    /// Represents the committer name and email address \
    /// Example: "Alice | alice@example.com"
    author: String,

    /// Represents the commit message \
    /// Example: "initial commit"
    message: String,

    /// Represents the commit time as a Unix timestamp - number of seconds
    /// since January 1, 1970 (UTC)
    timestamp: u64,

    /// Represents the tree hash of this commit
    tree: String,

    /// Represents the hash of the parent commit (the hash of the commit which preceded this commit)
    parent: Option<String>,

    /// Represents the commit hash
    hash: String,
}


/// Represents a staged changes object, which
/// records file changes which are not committed yet
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StagedFiles {
    /// holds the relative paths of the changed objects
    files: Vec<PathBuf>
}
