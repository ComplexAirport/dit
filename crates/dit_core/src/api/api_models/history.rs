use crate::models::Commit;

#[derive(Debug, Clone, Default)]
pub struct History {
    /// Represents the collection of commits in reverse order
    /// (from the child to the parent commits)
    pub commits: Vec<Commit>,
}

impl History {
    pub fn from(commits: Vec<Commit>) -> Self {
        Self {
            commits
        }
    }
}

