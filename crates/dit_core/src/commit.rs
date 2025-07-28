//! This module manages the commits in the Dit version control system
//!
//! A commit is a snapshot of a project's files at a given point in time,
//! along with metadata describing the relative to the last commit.
//! The metadata includes the author who wrote the changes, the parent
//! commit, the commit message, etc.

use crate::dit_project::DitProject;
use crate::stage::StagedFiles;
use crate::tree::TreeMgr;
use crate::errors::{DitResult, CommitError, OtherError, FsError};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::rc::Rc;
use std::time::SystemTime;

/// Manages the commits in our Dit version control system
pub struct CommitMgr {
    project: Rc<DitProject>,

    /// Represents the tree manager [`TreeMgr`]
    pub(crate) tree_mgr: TreeMgr,
}

/// Constructors
impl CommitMgr {
    pub fn from(project: Rc<DitProject>) -> Self {
        let tree_mgr = TreeMgr::from(project.clone());
        Self { project, tree_mgr }
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
    ) -> DitResult<String> {
        let parent_tree_hash = if let Some(parent_commit_hash) = &parent_commit_hash {
            let parent_commit = self.get_commit(parent_commit_hash)?;
            Some(parent_commit.tree)
        } else {
            None
        };

        let tree_hash = self.tree_mgr.create_tree(staged_files, parent_tree_hash)?;

        let timestamp = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .map_err(|_| OtherError::TimeWentBackwardsError)?
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
    pub fn get_commit<S: AsRef<str>>(&self, hash: S) -> DitResult<Commit> {
        self.load_commit(hash)
    }

    /// Checks whether a commit is a direct or indirect parent to another commit. This basically
    /// checks if the parent commit is reachable from the child commit.
    pub fn is_parent<S1, S2>(&self, parent: S1, child: S2)
        -> DitResult<bool>
    where
        S1: AsRef<str>,
        S2: AsRef<str>,
    {
        let parent = parent.as_ref();
        let child = child.as_ref();

        let mut current = Some(child.to_string());
        while let Some(head) = current {
            let commit = self.get_commit(head)?;
            if commit.hash == parent {
                return Ok(true);
            }
            current = commit.parent;
        }

        Ok(false)
    }
}

/// Private helper methods
impl CommitMgr {
    /// Writes the given commit to the commits directory
    fn write_commit(&self, commit: &Commit) -> DitResult<()> {
        let serialized = serde_json::to_string_pretty(&commit)
            .map_err(|_| CommitError::SerializationError(commit.hash.clone()))?;

        let path = self.project.commits().join(&commit.hash);
        std::fs::write(&path, &serialized)
            .map_err(|_| FsError::FileWriteError(path.display().to_string()))?;
        Ok(())
    }

    /// Reads and returns a commit given the commit's hash
    fn load_commit<S: AsRef<str>>(&self, hash: S) -> DitResult<Commit> {
        let hash = hash.as_ref();
        let path = self.project.commits().join(hash);

        let serialized = std::fs::read_to_string(&path)
            .map_err(|_| FsError::FileReadError(path.display().to_string()))?;

        let mut commit: Commit = serde_json::from_str(&serialized)
            .map_err(|_| CommitError::DeserializationError(hash.to_string()))?;

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
