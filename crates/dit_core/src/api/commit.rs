use crate::Dit;
use crate::errors::DitResult;
use crate::models::Commit;

/// Manipulate commits
impl Dit {
    /// Creates a commit based on a message and an author
    pub fn commit<S1: Into<String>, S2: Into<String>>(&mut self, author: S1, message: S2)
                                                      -> DitResult<()>
    {
        self.commit_mgr.borrow_mut().create_commit(
            author,
            message,
            &mut self.tree_mgr.borrow_mut(),
            &mut self.stage_mgr.borrow_mut(),
            &mut self.branch_mgr.borrow_mut(),
        )
    }

    /// Performs a mixed reset to a specific commit. All files not included in
    /// that commit tree stay the same.
    pub fn mixed_reset<S: AsRef<str>>(&mut self, commit: S) -> DitResult<()>
    {
        self.commit_mgr.borrow_mut().mixed_reset(
            commit,
            &mut self.blob_mgr.borrow_mut(),
            &mut self.tree_mgr.borrow_mut(),
            &mut self.branch_mgr.borrow_mut()
        )
    }

    /// Performs a hard reset to a specific commit. The root of the project will be changed
    /// to exactly match the target commit tree (except the ignored files)
    pub fn hard_reset<S: AsRef<str>>(&mut self, commit: S) -> DitResult<()>
    {
        self.commit_mgr.borrow_mut().hard_reset(
            commit,
            &mut self.blob_mgr.borrow_mut(),
            &mut self.tree_mgr.borrow_mut(),
            &mut self.branch_mgr.borrow_mut(),
            &mut self.ignore_mgr.borrow_mut(),
        )
    }

    /// Performs a soft reset to a specific commit. Only changes the head pointer and leaves
    /// the files untouched
    pub fn soft_reset<S: AsRef<str>>(&mut self, commit: S) -> DitResult<()> {
        self.commit_mgr.borrow_mut().soft_reset(commit, &mut self.branch_mgr.borrow_mut())
    }
}


/// Getters
impl Dit {
    /// Returns the commit history
    pub fn get_history(&mut self, mut count: isize) -> DitResult<Vec<Commit>> {
        if count < 0 {
            count = isize::MAX;
        }

        let mut commits = Vec::new();
        let mut head_commit = self.branch_mgr.borrow().get_head_commit().cloned();

        while let Some(head) = &head_commit {
            if count == 0 {
                break;
            }

            let commit = self.commit_mgr.borrow().get_commit(head)?;
            head_commit = commit.parents.first().cloned();
            commits.push(commit);

            count -= 1;
        }

        Ok(commits)
    }
}