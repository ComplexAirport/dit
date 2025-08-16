use crate::Repo;
use crate::errors::DitResult;
use std::sync::Arc;
use ignore::gitignore::Gitignore;

pub(super) const DEFAULT_IGNORE_LIST: &[&str] = &[".dit"];

pub struct IgnoreMgr {
    pub(super) repo: Arc<Repo>,

    /// List of the ignored files and directories (not the ignored patterns)
    pub(super) ignore: Gitignore,
}

impl IgnoreMgr {
    pub fn from(repo: Arc<Repo>) -> DitResult<Self> {
        let mut ignore_mgr = Self {
            repo,
            ignore: Gitignore::empty(),
        };

        Self::load(&mut ignore_mgr)?;

        Ok(ignore_mgr)
    }
}