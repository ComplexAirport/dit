use crate::Repo;
use crate::errors::DitResult;
use std::path::PathBuf;
use std::sync::Arc;

pub(super) const DEFAULT_IGNORE_LIST: &[&str] = &[".dit"];

pub struct IgnoreMgr {
    pub(super) repo: Arc<Repo>,

    /// List of the ignored files and directories (not the ignored patterns)
    pub(super) ignored_list: Vec<PathBuf>,
}

impl IgnoreMgr {
    pub fn from(repo: Arc<Repo>) -> DitResult<Self> {
        let mut ignore_mgr = Self {
            repo,
            ignored_list: Vec::new(),
        };

        Self::load(&mut ignore_mgr)?;

        Ok(ignore_mgr)
    }
}