use crate::errors::DitResult;
use crate::helpers::{DitModel, DitModelDefault};
use crate::managers::config::ConfigMgr;
use crate::models::Config;
use crate::Repo;

impl ConfigMgr {
    pub(super) fn read(repo: &Repo) -> DitResult<Config> {
        Config::deserialize_default_from(repo.config_file())
    }

    pub(super) fn store(&self) -> DitResult<()> {
        self.config.serialize_to(self.repo.config_file())
    }
}
