use crate::managers::ignore::manager::IgnoreMgr;
use crate::helpers::{expand_glob, read_to_string, write_to_file_truncate};
use crate::errors::DitResult;

/// Read and write to the .ditignore file
impl IgnoreMgr {
    /// Load the ignored files and directories from [`IGNORE_FILE`]
    ///
    /// [`IGNORE_FILE`]: crate::api::dit_component_paths::IGNORE_FILE
    pub(super) fn load(&mut self) -> DitResult<()> {
        self.ignored_patterns = self.get_ignored_patterns()?;

        let ignored_list = self.ignored_patterns
            .iter()
            .map(|pat| expand_glob(self.repo.repo_path(), pat))
            .collect::<DitResult<Vec<_>>>()?
            .into_iter()
            .flatten()
            .collect();

        self.ignored_list = ignored_list;

        Ok(())
    }


    /// Stores the current ignored files and directories (from self)
    /// to [`IGNORE_FILE`]
    ///
    /// [`IGNORE_FILE`]: crate::api::dit_component_paths::IGNORE_FILE
    pub(super) fn store(&self) -> DitResult<()> {
        let content = self.ignored_patterns.join("\n");

        write_to_file_truncate(
            self.repo.ignore_file(),
            content,
        )?;

        Ok(())
    }
}

/// Getters
impl IgnoreMgr {
    /// Returns the list of the ignored patterns
    pub fn get_ignored_patterns(&self) -> DitResult<Vec<String>> {
        let ignore_file = self.repo.ignore_file();

        Ok(read_to_string(ignore_file)?
            .lines()
            .map(str::trim)
            .map(String::from)
            .filter(|pat| !pat.is_empty())
            .collect())
    }
}
