use crate::Repo;
use crate::models::Config;
use crate::errors::DitResult;
use std::sync::Arc;

pub struct ConfigMgr {
    pub(super) repo: Arc<Repo>,

    pub(super) config: Config,
}

impl ConfigMgr {
    pub fn from(repo: Arc<Repo>) -> DitResult<Self> {
        let config = Self::read(&repo)?;

        Ok(Self {
            repo,
            config,
        })
    }
}
