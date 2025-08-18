use crate::managers::ignore::manager::IgnoreMgr;
use crate::helpers::ignore_from_file;
use crate::errors::DitResult;
use std::sync::Arc;

/// Read and write to the .ditignore file
impl IgnoreMgr {
    /// Load the ignored files and directories from [`IGNORE_FILE`]
    ///
    /// [`IGNORE_FILE`]: crate::api::dit_component_paths::IGNORE_FILE
    pub(super) fn load(&mut self) -> DitResult<()> {
        self.ignore = Arc::new(ignore_from_file(
            self.repo.repo_path(),
            self.repo.ignore_file(),
        )?);
        Ok(())
    }
}
