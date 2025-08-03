//! This module manages the commits in the Dit version control system
//!
//! A commit is a snapshot of a project's files at a given point in time,
//! along with metadata describing the relative to the last commit.
//! The metadata includes the author who wrote the changes, the parent
//! commit, the commit message, etc.

use std::collections::{HashSet, VecDeque};
use crate::repo::Repo;
use crate::tree::TreeMgr;
use crate::blob::BlobMgr;
use crate::branch::BranchMgr;
use crate::stage::StageMgr;
use crate::errors::{DitResult, OtherError};
use crate::helpers::clear_dir_except;
use crate::models::{Commit, Stage};
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
            author, message, stage_mgr.stage(), parent, tree_mgr
        )?;

        branch_mgr.set_head_commit(commit_hash)?;

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
    pub fn get_commit<S: Into<String>>(&self, hash: S) -> DitResult<Commit> {
        self.load_commit(hash)
    }

    /// Tries to find a common ancestor for two commits
    pub fn common_ancestor<S1, S2>(&self, a: S1, b: S2) -> DitResult<Option<String>>
    where
        S1: Into<String>,
        S2: Into<String>,
    {
        let commit_a = a.into();
        let commit_b = b.into();

        let mut visited_a = HashSet::new();
        let mut visited_b = HashSet::new();

        let mut queue_a = VecDeque::from([commit_a]);
        let mut queue_b = VecDeque::from([commit_b]);

        while !queue_a.is_empty() && !queue_b.is_empty() {
            if let Some(current_a) = queue_a.pop_front() {
                if visited_b.contains(&current_a) {
                    return Ok(Some(current_a));
                }

                if visited_a.insert(current_a.clone()) {
                    let parent = self.get_parent(current_a)?;
                    if let Some(parent) = parent {
                        queue_a.push_back(parent);
                    }
                }
            }

            if let Some(current_b) = queue_b.pop_front() {
                if visited_a.contains(&current_b) {
                    return Ok(Some(current_b));
                }

                if visited_b.insert(current_b.clone()) {
                    let parent = self.get_parent(current_b)?;
                    if let Some(parent) = parent {
                        queue_b.push_back(parent);
                    }
                }
            }
        }

        Ok(None)
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
    fn load_commit<S: Into<String>>(&self, hash: S) -> DitResult<Commit> {
        let hash = hash.into();
        let path = self.repo.commits().join(&hash);

        let mut commit = Commit::read_from(path)?;

        commit.hash = hash;

        Ok(commit)
    }

    /// Returns the parent commit hash of a given commit
    fn get_parent<S: Into<String>>(&self, hash: S) -> DitResult<Option<String>> {
        let hash = hash.into();
        let path = self.repo.commits().join(&hash);
        let commit = Commit::read_from(path)?;
        Ok(commit.parent)
    }

    /// Creates a commit and returns the commit hash
    fn create_commit_inner(
        &self,
        author: String,
        message: String,
        stage: &Stage,
        parent_commit_hash: Option<String>,
        tree_mgr: &mut TreeMgr,
    ) -> DitResult<String> {
        let parent_tree_hash = if let Some(parent_commit_hash) = &parent_commit_hash {
            let parent_commit = self.get_commit(parent_commit_hash)?;
            Some(parent_commit.tree)
        } else {
            None
        };

        let tree_hash = tree_mgr.create_tree(stage, parent_tree_hash)?;

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
