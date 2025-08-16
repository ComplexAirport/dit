use crate::Dit;
use crate::api_models::status::{ChangeType, Status};
use crate::errors::DitResult;

impl Dit {
    /// Returns the current dit status (tracked/untracked files, etc.)
    pub fn get_status(&self) -> DitResult<Status> {
        let tree_mgr = self.tree_mgr().borrow();
        let ignore_mgr = self.ignore_mgr()?.borrow();
        let index_mgr = self.index_mgr()?.borrow();
        let commit_mgr = self.commit_mgr().borrow();
        let branch_mgr = self.branch_mgr()?.borrow();

        let mut status = Status::new();
        let tracked_changes = index_mgr.get_all_tracked_changes(
            &tree_mgr, &commit_mgr, &branch_mgr)?;
        let untracked_changes = index_mgr.get_all_untracked_changes(&ignore_mgr)?;

        for (rel_path, change) in tracked_changes {
            status.add_tracked(rel_path, ChangeType::from(change));
        }

        for (rel_path, change) in untracked_changes {
            status.add_untracked(rel_path, ChangeType::from(change));
        }

        Ok(status)
    }
}
