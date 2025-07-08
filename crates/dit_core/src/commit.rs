//! This module manages the commits in the Dit version control system
//!
//! A commit is a snapshot of a project's files at a given point in time,
//! along with metadata describing the relative to the last commit.
//! The metadata includes the author who wrote the changes, the parent
//! commit, the commit message, etc.

use crate::dit_project::DitProject;
use crate::stage::StagedFiles;
use crate::tree::TreeMgr;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::io;
use std::rc::Rc;
use std::time::SystemTime;

/// Manages the commits in our Dit version control system
pub struct CommitMgr {
    project: Rc<DitProject>,

    /// Represents the tree manager [`TreeMgr`]
    tree_mgr: TreeMgr,
}

/// Constructors
impl CommitMgr {
    pub fn from(project: Rc<DitProject>) -> io::Result<Self> {
        let tree_mgr = TreeMgr::from(project.clone())?;
        Ok(Self { project, tree_mgr })
    }
}

/// Manage commits
impl CommitMgr {
    /// Creates a commit and returns the commit hash
    pub fn create_commit(
        &self,
        author: String,
        message: String,
        staged_files: &StagedFiles,
        parent_commit_hash: Option<String>,
    ) -> io::Result<String> {
        let parent_tree_hash = if let Some(parent_commit_hash) = &parent_commit_hash {
            let parent_commit = self.get_commit(parent_commit_hash)?;
            Some(parent_commit.tree)
        } else {
            None
        };

        let tree_hash = self.tree_mgr.create_tree(staged_files, parent_tree_hash)?;

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


    /// Returns a commit by hash
    pub fn get_commit<S: AsRef<str>>(&self, hash: S) -> io::Result<Commit> {
        self.load_commit(hash)
    }
}

/// Private helper methods
impl CommitMgr {
    /// Writes the given commit to the commits directory
    fn write_commit(&self, commit: &Commit) -> io::Result<()> {
        let serialized = serde_json::to_string_pretty(&commit)?;
        let path = self.project.commits().join(&commit.hash);
        std::fs::write(path, serialized)?;
        Ok(())
    }

    /// Reads and returns a commit given the commit's hash
    fn load_commit<S: AsRef<str>>(&self, hash: S) -> io::Result<Commit> {
        let hash = hash.as_ref();
        let path = self.project.commits().join(hash);
        let serialized = std::fs::read_to_string(path)?;
        let mut commit: Commit = serde_json::from_str(&serialized)?;
        commit.hash = hash.to_string();
        Ok(commit)
    }
}

/// Represents a commit object
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Commit {
    /// Represents the committer name and email address \
    /// Example: "Alice | alice@example.com"
    pub author: String,

    /// Represents the commit message \
    /// Example: "initial commit"
    pub message: String,

    /// Represents the commit time as a Unix timestamp - number of seconds
    /// since January 1, 1970 (UTC)
    pub timestamp: u64,

    /// Represents the tree hash of this commit
    pub tree: String,

    /// Represents the hash of the parent commit (the hash of the commit which preceded this commit)
    pub parent: Option<String>,

    /// Represents the commit hash
    #[serde(skip)]
    pub hash: String,
}
