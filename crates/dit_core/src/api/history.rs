use crate::Dit;
use crate::api_models::history::History;
use crate::errors::DitResult;

impl Dit {
    /// Returns the commit history
    pub fn get_history(&mut self, mut count: isize) -> DitResult<History> {
        if count < 0 {
            count = isize::MAX;
        }

        let mut commits = Vec::new();
        let mut head_commit = self.branch_mgr()?.borrow().get_head_commit().cloned();

        while let Some(head) = &head_commit {
            if count == 0 {
                break;
            }

            let commit = self.commit_mgr().borrow().get_commit(head)?;
            head_commit = commit.parents.first().cloned();
            commits.push(commit);

            count -= 1;
        }

        let history = History::from(commits);
        Ok(history)
    }
}
