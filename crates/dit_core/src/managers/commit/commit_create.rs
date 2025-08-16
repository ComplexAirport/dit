use crate::managers::branch::BranchMgr;
use crate::managers::commit::CommitMgr;
use crate::managers::index::IndexMgr;
use crate::managers::tree::TreeMgr;
use crate::models::{Commit, Index};
use crate::helpers::DitHasher;
use crate::errors::{DitResult, OtherError};
use std::time::SystemTime;

/// Public
impl CommitMgr {
    /// Commits the changes given the commit author and the message
    pub fn create_commit<S1, S2>(
        &mut self,
        author: S1,
        message: S2,
        tree_mgr: &mut TreeMgr,
        index_mgr: &mut IndexMgr,
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
            author, message, index_mgr.index(), parent, tree_mgr,
        )?;

        branch_mgr.set_head_commit(commit_hash)
    }
}


/// Private
impl CommitMgr {

    /// Creates a commit and returns the commit hash
    fn create_commit_inner(
        &self,
        author: String,
        message: String,
        index: &Index,
        parent_commit_hash: Option<String>,
        tree_mgr: &TreeMgr,
    ) -> DitResult<String> {
        let tree_hash = tree_mgr.create_tree(index.clone())?;

        let timestamp = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .map_err(|_| OtherError::TimeWentBackwardsError)?
            .as_secs();

        let mut hasher = DitHasher::new();
        hasher.update(author.as_bytes());
        hasher.update(message.as_bytes());
        hasher.update(&timestamp.to_le_bytes());
        hasher.update(tree_hash.as_bytes());
        hasher.update(&parent_commit_hash.clone()
                          .map(|s| s.into_bytes())
                          .unwrap_or_else(|| vec![0]), );
        let commit_hash = hasher.finalize_string();

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
