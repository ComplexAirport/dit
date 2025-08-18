use crate::Repo;
use crate::errors::DitResult;
use std::sync::Arc;
use ignore::gitignore::Gitignore;

pub(super) const DEFAULT_IGNORE_LIST: &[&str] = &[".dit"];

pub struct IgnoreMgr {
    pub(super) repo: Arc<Repo>,

    pub(super) ignore: Arc<Gitignore>,
}

impl IgnoreMgr {
    pub fn from(repo: Arc<Repo>) -> DitResult<Self> {
        let mut ignore_mgr = Self {
            repo,
            ignore: Arc::new(Gitignore::empty()),
        };

        Self::load(&mut ignore_mgr)?;

        Ok(ignore_mgr)
    }
}