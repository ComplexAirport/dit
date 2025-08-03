use crate::managers::commit::CommitMgr;

/// Iterates through ancestors of a commit
pub struct CommitIterator<'a> {
    current_commit: Option<String>,

    commit_mgr: &'a CommitMgr,
}

impl<'a> CommitIterator<'a> {
    pub fn new<S: Into<String>>(
        current_commit: S,
        commit_mgr: &'a CommitMgr
    ) -> Self
    {
        let current_commit = Some(current_commit.into());
        Self {
            current_commit,
            commit_mgr,
        }
    }
}

impl<'a> Iterator for CommitIterator<'a> {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        self.current_commit.as_ref()?;
        let current_commit = self.current_commit.take().unwrap();

        let current = self.commit_mgr.get_commit(current_commit);
        if current.is_err() {
            return None;
        }
        let current = current.unwrap();

        let parent = current.parent;
        self.current_commit = parent;
        Some(current.hash)
    }
}
