use crate::Repo;
use crate::errors::DitResult;
use std::path::PathBuf;
use std::sync::Arc;

pub(super) const DEFAULT_IGNORE_LIST: &[&str] = &[/* ".dit" */];

pub struct IgnoreMgr {
    pub(super) repo: Arc<Repo>,

    pub(super) ignore_list: Vec<PathBuf>
}

impl IgnoreMgr {
    pub fn from(repo: Arc<Repo>) -> DitResult<Self> {
        let mut ignore_mgr = Self {
            repo,
            ignore_list: Vec::new()
        };

        Self::load(&mut ignore_mgr)?;

        Ok(ignore_mgr)
    }
}