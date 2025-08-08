use crate::errors::DitResult;
use crate::helpers::{expand_glob, path_to_string, read_to_string, remove_file, write_to_file};
use crate::managers::ignore::manager::IgnoreMgr;

/// Read and write to the .ditignore file
impl IgnoreMgr {
    /// Load the ignored files and directories from [`IGNORE_FILE`]
    ///
    /// [`IGNORE_FILE`]: crate::api::dit_component_paths::IGNORE_FILE
    pub(super) fn load(&mut self) -> DitResult<()> {
        let repo_path = self.repo.repo_path();
        let ignore_file = self.repo.ignore_file();

        if !ignore_file.is_file() {
            self.ignore_list = Vec::new();
            return Ok(());
        }

        let ignore_list = read_to_string(ignore_file)?
            .lines()
            .map(str::trim)
            .filter(|pat| !pat.is_empty())
            .map(|pat| expand_glob(repo_path, pat))
            .collect::<DitResult<Vec<_>>>()?
            .into_iter()
            .flatten()
            .collect();

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

        let ignore_file = self.repo.ignore_file();
        if ignore_file.is_file() {
            remove_file(self.repo.ignore_file())?;
        }
        write_to_file(
            self.repo.ignore_file(),
            content,
        )?;

        Ok(())
    }
}
