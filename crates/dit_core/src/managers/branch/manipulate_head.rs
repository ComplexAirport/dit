use crate::managers::branch::BranchMgr;
use crate::errors::DitResult;

/// Branch head operations
impl BranchMgr {
    /// Sets the current (head) branch to a new value
    pub fn set_current_branch<S: AsRef<str>>(&mut self, branch: S) -> DitResult<()> {
        let branch = branch.as_ref();
        self.curr_branch = Some(branch.to_string());
        self.store()?;
        Ok(())
    }

    /// Returns the name of the current (head) branch
    pub fn get_current_branch(&self) -> Option<&String> {
        self.curr_branch.as_ref()
    }
}

/// Commit head operations
impl BranchMgr {
    /// Sets the current (head) commit to a new value
    pub fn set_head_commit<S: AsRef<str>>(&mut self, commit: S) -> DitResult<()> {
        let commit = commit.as_ref();
        self.curr_commit = Some(commit.to_string());
        self.store()?;
        Ok(())
    }

    /// Returns the hash of the current commit
    pub fn get_head_commit(&self) -> Option<&String> { self.curr_commit.as_ref() }
}
