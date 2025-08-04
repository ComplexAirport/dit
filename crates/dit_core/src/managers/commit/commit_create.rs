use crate::managers::branch::BranchMgr;
use crate::managers::commit::CommitMgr;
use crate::managers::stage::StageMgr;
use crate::managers::tree::TreeMgr;
use crate::errors::{DitResult, OtherError};
use crate::models::{Commit, Stage};
use sha2::{Digest, Sha256};
use std::time::SystemTime;

/// Public
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
}


/// Private
impl CommitMgr {

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

        let parents = if let Some(parent) = parent_commit_hash {
            vec![parent]
        } else {
            vec![]
        };

        let commit = Commit {
            author,
            message,
            timestamp,
            tree: tree_hash,
            parents,
            hash: commit_hash.clone(),
        };

        self.write_commit(&commit)?;

        Ok(commit_hash)
    }
}
