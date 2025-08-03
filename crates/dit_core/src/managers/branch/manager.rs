use crate::repo::Repo;
use crate::errors::DitResult;
use std::rc::Rc;

pub struct BranchMgr {
    pub(super) repo: Rc<Repo>,

    /// Represents the current branch name
    pub(super) curr_branch: Option<String>,

    /// Represents the current commit head (the hash of the current commit)
    pub(super) curr_commit: Option<String>,
}

/// Constructors
impl BranchMgr {
    pub fn from(repo: Rc<Repo>) -> DitResult<Self> {
        let mut branch_mgr = Self {
            repo,
            curr_branch: None,
            curr_commit: None,
        };

        Self::load(&mut branch_mgr)?;

        Ok(branch_mgr)
    }
}
