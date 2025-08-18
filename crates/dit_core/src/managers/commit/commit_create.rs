use crate::managers::config::ConfigMgr;
use crate::managers::branch::BranchMgr;
use crate::managers::commit::CommitMgr;
use crate::managers::index::IndexMgr;
use crate::managers::tree::TreeMgr;
use crate::models::Commit;
use crate::helpers::DitHasher;
use crate::errors::{DitResult, OtherError};
use std::time::SystemTime;

/// Public
impl CommitMgr {
    /// Commits the changes given the commit author and the message
    pub fn create_commit<S1: Into<String>, S2: Into<String>>(
        &mut self,
        message: S1,
        author: Option<S2>,
        tree_mgr: &TreeMgr,
        index_mgr: &IndexMgr,
        branch_mgr: &mut BranchMgr,
        config_mgr: &ConfigMgr,
    ) -> DitResult<()> {
        let parent = branch_mgr.get_head_commit().cloned();
        let author = match author {
            Some(author) => author.into(),
            None => config_mgr.require_user()?,
        };
        let message = message.into();
        let index = index_mgr.index().clone();

        let tree_hash = tree_mgr.create_tree(index)?;

        let timestamp = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .map_err(|_| OtherError::TimeWentBackwardsError)?
            .as_secs();

        let mut hasher = DitHasher::new();
        hasher.update(author.as_bytes());
        hasher.update(message.as_bytes());
        hasher.update(&timestamp.to_le_bytes());
        hasher.update(tree_hash.as_bytes());
        hasher.update(&parent.clone()
            .map(|s| s.into_bytes())
            .unwrap_or_else(|| vec![0]), );
        let commit_hash = hasher.finalize_string();

        let parents = if let Some(parent) = parent {
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

        branch_mgr.set_head_commit(commit_hash)
    }
}
