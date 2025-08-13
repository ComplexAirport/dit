use crate::Dit;
use crate::errors::DitResult;

/// Manipulate branches
impl Dit {
    /// Creates a new branch
    pub fn create_branch<S: AsRef<str>>(&mut self, name: S) -> DitResult<()> {
        self.branch_mgr()?.borrow_mut().create_branch(name)
    }

    /// Switches to a different branch
    pub fn switch_branch<S: AsRef<str>>(&mut self, name: S, is_hard: bool) -> DitResult<()> {
        self.branch_mgr()?.borrow_mut().switch_branch(
            name,
            is_hard,
            &mut self.blob_mgr().borrow_mut(),
            &mut self.tree_mgr().borrow_mut(),
            &mut self.commit_mgr().borrow_mut(),
            &mut self.stage_mgr()?.borrow_mut(),
            &mut self.ignore_mgr()?.borrow_mut(),
        )
    }

    /// Merge a branch into the current branch
    pub fn merge_branch<S: AsRef<str>>(&mut self, branch: S) -> DitResult<()> {
        self.branch_mgr()?.borrow_mut().merge_to(
            branch,
            &self.commit_mgr().borrow()
        )
    }

    /// Removes a given branch
    pub fn remove_branch<S: AsRef<str>>(&mut self, name: S) -> DitResult<()> {
        self.branch_mgr()?.borrow_mut().remove_branch(name)
    }
}


/// Getters
impl Dit {
    /// Returns the name of the current branch
    pub fn get_branch(&self) -> DitResult<Option<String>> {
        Ok(self.branch_mgr()?.borrow().get_current_branch().cloned())
    }

    /// Returns the hash of the current HEAD commit
    pub fn get_head_commit(&self) -> DitResult<Option<String>> {
        Ok(self.branch_mgr()?.borrow().get_head_commit().cloned())
    }
}
