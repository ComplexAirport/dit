use crate::errors::DitResult;
use crate::helpers::{path_to_string, read_to_string, write_to_file};
use crate::managers::ignore::manager::IgnoreMgr;
use std::path::PathBuf;

/// Read and write to the .ditignore file
impl IgnoreMgr {
    /// Load the ignored files and directories from [`IGNORE_FILE`]
    ///
    /// [`IGNORE_FILE`]: crate::api::dit_component_paths::IGNORE_FILE
    pub(super) fn load(&mut self) -> DitResult<()> {
        let ignore_file = self.repo.ignore_file();

        if !ignore_file.is_file() {
            self.ignore_list = Vec::new();
            return Ok(());
        }

        let ignore_list: Vec<PathBuf> = read_to_string(ignore_file)?
            .lines()
            .map(|p| PathBuf::from(p.trim()))
            .map(|p| self.repo.abs_path_from_repo(p, true))
            .collect::<DitResult<_>>()?;

        self.ignore_list = ignore_list;

        Ok(())
    }

    /// Stores the current ignored files and directories (from self)
    /// to [`IGNORE_FILE`]
    ///
    /// [`IGNORE_FILE`]: crate::api::dit_component_paths::IGNORE_FILE
    pub(super) fn store(&self) -> DitResult<()> {
        let content = self.ignore_list
            .iter()
            .map(path_to_string)
            .collect::<Vec<String>>()
            .join("\n");

        write_to_file(
            self.repo.ignore_file(),
            content,
        )?;

        Ok(())
    }
}
