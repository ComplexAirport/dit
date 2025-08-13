use crate::Dit;
use crate::managers::branch::BranchMgr;
use crate::managers::commit::CommitMgr;
use crate::managers::stage::StageMgr;
use crate::managers::tree::TreeMgr;
use crate::models::ChangeType;
use crate::api_models::status::{ChangeType as ApiChangeType, Status};
use crate::errors::DitResult;
use std::path::PathBuf;

impl Dit {
    /// Returns the current dit status (tracked/untracked files, etc.)
    pub fn get_status(&self) -> DitResult<Status> {
        let ignore_mgr = self.ignore_mgr()?.borrow();
        let stage_mgr = self.stage_mgr()?.borrow();
        let tree_mgr = self.tree_mgr().borrow();
        let commit_mgr = self.commit_mgr().borrow();
        let branch_mgr = self.branch_mgr()?.borrow();

        // First, get the list of the staged files
        let mut staged_files = stage_mgr.get_stage().files.clone();

        // Then, get the previous tree files
        let tree = branch_mgr.get_head_tree(&tree_mgr, &commit_mgr)?;
        let mut tree_files = tree.map(|t| t.files).unwrap_or_default();

        let mut status = Status::new();
        ignore_mgr.walk_dir_files(self.repo.repo_path(), |abs_path| {
            let rel_path = self.repo.rel_path(&abs_path)?;
            tree_files.remove(&rel_path);
            staged_files.remove(&rel_path);
            self.register_changes(
                rel_path, &mut status,
                &tree_mgr, &commit_mgr,
                &stage_mgr, &branch_mgr,
            )?;
            Ok(())
        })?;

        // Now check for files left in the tree and in the stage. These files were deleted
        for rel_path in tree_files.keys() {
            self.register_changes(
                rel_path, &mut status,
                &tree_mgr, &commit_mgr,
                &stage_mgr, &branch_mgr,
            )?;
        }

        for rel_path in staged_files.keys() {
            self.register_changes(
                rel_path,
                &mut status,
                &tree_mgr,
                &commit_mgr,
                &stage_mgr,
                &branch_mgr,
            )?;
        }

        Ok(status)
    }
}

/// Private
impl Dit {
    fn register_changes<P: Into<PathBuf>>(
        &self,
        rel_path: P,
        status: &mut Status,
        tree_mgr: &TreeMgr,
        commit_mgr: &CommitMgr,
        stage_mgr: &StageMgr,
        branch_mgr: &BranchMgr,
    ) -> DitResult<()> {
        let rel_path = rel_path.into();
        let (untracked, tracked) = stage_mgr.get_changes(
            rel_path.clone(), tree_mgr, commit_mgr, branch_mgr
        )?;

        if !matches!(untracked, ChangeType::Unchanged) {
            status.add_untracked(rel_path.clone(), ApiChangeType::from(untracked));
        }

        if !matches!(tracked, ChangeType::Unchanged) {
            status.add_tracked(rel_path, ApiChangeType::from(tracked));
        }

        Ok(())
    }
}
