//! This module manages the commits in the Dit version control system
//!
//! A commit is a snapshot of a project's files at a given point in time,
//! along with metadata describing the relative to the last commit.
//! The metadata includes the author who wrote the changes, the parent
//! commit, the commit message, etc.

use crate::repo::Repo;
use crate::tree::TreeMgr;
use crate::blob::BlobMgr;
use crate::branch::BranchMgr;
use crate::stage::{StageMgr, StagedFiles};
use crate::errors::{DitResult, CommitError, OtherError, FsError};
use crate::helpers::clear_dir_except;
use crate::models::Commit;
use sha2::{Digest, Sha256};
use std::rc::Rc;
use std::time::SystemTime;

/// Manages the commits in our Dit version control system
pub struct CommitMgr {
    repo: Rc<Repo>,
}

/// Constructors
impl CommitMgr {
    pub fn from(repo: Rc<Repo>) -> Self {
        Self { repo }
    }
}


/// Manage commits
impl CommitMgr {
    /// Commits the changes given the commit author and the message
    pub fn create_commit<S1, S2>(
        &mut self,
        author: S1,
        message: S2,
        blob_mgr: &mut BlobMgr,
        tree_mgr: &mut TreeMgr,
        stage_mgr: &mut StageMgr,
        branch_mgr: &mut BranchMgr,
    ) -> DitResult<()>
    where
        S1: Into<String>,
        S2: Into<String>,
    {
        let parent = branch_mgr.get_head_commit().cloned();
        let author = author.into();
        let message = message.into();

        let commit_hash = self.create_commit_inner(
            author, message, stage_mgr.staged_files(), parent, blob_mgr, tree_mgr
        )?;

        branch_mgr.set_head_commit(commit_hash)?;

        // Clean up the stage
        stage_mgr.clear_stage()?;

        Ok(())
    }

    /// Performs a soft reset to a specific commit. Only changes the head
    pub fn soft_reset<S: AsRef<str>>(
        &mut self,
        commit: S,
        branch_mgr: &mut BranchMgr,
    ) -> DitResult<()> {
        let commit = self.get_commit(commit.as_ref())?;
        branch_mgr.set_head_commit(commit.hash)?;
        Ok(())
    }

    /// Performs a mixed reset to a specific commit. Mixed reset means that the files
    /// not included in that commit tree stay the same.
    pub fn mixed_reset<S: AsRef<str>>(
        &mut self,
        commit: S,
        blob_mgr: &mut BlobMgr,
        tree_mgr: &mut TreeMgr,
        branch_mgr: &mut BranchMgr,
    ) -> DitResult<()> {
        let commit = self.get_commit(commit.as_ref())?;

        tree_mgr.recover_tree(commit.tree, blob_mgr)?;
        branch_mgr.set_head_commit(commit.hash)?;

        Ok(())
    }

    pub fn hard_reset<S: AsRef<str>>(
        &mut self,
        commit: S,
        blob_mgr: &mut BlobMgr,
        tree_mgr: &mut TreeMgr,
        branch_mgr: &mut BranchMgr,
    ) -> DitResult<()> {
        let commit = self.get_commit(commit.as_ref())?;

        // Clear the project directory to recover the target commit tree
        clear_dir_except(self.repo.repo_path(), self.repo.ignore())?;

        tree_mgr.recover_tree(commit.tree, blob_mgr)?;

        branch_mgr.set_head_commit(commit.hash)?;

        Ok(())
    }
}


/// Getters
impl CommitMgr {
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
        let path = self.repo.commits().join(&commit.hash);
        commit.write_to(path)?;
        Ok(())
    }

    /// Reads and returns a commit given the commit's hash
    fn load_commit<S: AsRef<str>>(&self, hash: S) -> DitResult<Commit> {
        let hash = hash.as_ref();
        let path = self.repo.commits().join(hash);

        let mut commit = Commit::read_from(path)?;

        commit.hash = hash.to_string();

        Ok(commit)
    }

    /// Creates a commit and returns the commit hash
    fn create_commit_inner(
        &self,
        author: String,
        message: String,
        staged_files: &StagedFiles,
        parent_commit_hash: Option<String>,
        blob_mgr: &mut BlobMgr,
        tree_mgr: &mut TreeMgr,
    ) -> DitResult<String> {
        let parent_tree_hash = if let Some(parent_commit_hash) = &parent_commit_hash {
            let parent_commit = self.get_commit(parent_commit_hash)?;
            Some(parent_commit.tree)
        } else {
            None
        };

        let tree_hash = tree_mgr.create_tree(staged_files, parent_tree_hash, blob_mgr)?;

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
}
