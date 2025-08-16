use crate::managers::ignore::manager::IgnoreMgr;
use crate::helpers::{expand_glob, read_to_string};
use crate::errors::DitResult;

/// Read and write to the .ditignore file
impl IgnoreMgr {
    /// Load the ignored files and directories from [`IGNORE_FILE`]
    ///
    /// [`IGNORE_FILE`]: crate::api::dit_component_paths::IGNORE_FILE
    pub(super) fn load(&mut self) -> DitResult<()> {
        let a = std::time::Instant::now();
        let ignored_list = read_to_string(self.repo.ignore_file())?
            .lines()
            .filter(|line| !line.trim().is_empty())
            .map(|pat| expand_glob(self.repo.repo_path(), pat))
            .collect::<DitResult<Vec<_>>>()?
            .into_iter()
            .flatten()
            .collect();
        println!("{:?}", a.elapsed());

        self.ignored_list = ignored_list;

        Ok(())
    }
}
