use crate::managers::blob::BlobMgr;
use crate::managers::tree::TreeMgr;
use crate::managers::commit::CommitMgr;
use crate::managers::branch::BranchMgr;
use crate::helpers::clear_dir_except;
use crate::errors::DitResult;

impl CommitMgr {
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
