use crate::managers::stage::StageMgr;
use crate::errors::DitResult;
use crate::helpers::{copy_with_hash_as_name, get_buf_reader, remove_file};
use std::path::Path;

impl StageMgr {
    /// Stages a file based on its path. IMPORTANT: files in the
    /// stage folder have their names as their content hashes.
    /// This way, the blob hash doesn't have to be recomputed
    /// when the file is commited
    pub fn stage_file<P: AsRef<Path>>(&mut self, file_path: P) -> DitResult<()> {
        let reader = get_buf_reader(&file_path)?;

        let hash = copy_with_hash_as_name(reader, self.repo.stage())?;
        let staged_file_path = self.repo.stage().join(&hash);

        let file_path = self.repo.rel_path(file_path)?;
        self.stage
            .files
            .insert(file_path.to_path_buf(), staged_file_path);
        self.update_stage_file()?;

        Ok(())
    }

    /// Unstages a file based on its path
    pub fn unstage_file<P: AsRef<Path>>(&mut self, file_path: P) -> DitResult<()> {
        let file_path = file_path.as_ref();
        let relative_path = self.repo.abs_path_from_cwd(file_path, false)?;

        let staged_path = self.stage.files.remove(&relative_path);

        if let Some(staged_path) = staged_path {
            remove_file(&staged_path)?;
        }

        self.update_stage_file()?;

        Ok(())
    }

    /// Clears all staged files and clears the [`STAGE_FILE`]
    ///
    /// - `remove_files` - specifies whether to remove files from the filesystem or only
    ///   update the inner state and the stage file
    ///
    /// [`STAGE_FILE`]: crate::project_structure::STAGE_FILE
    pub fn clear_stage(&mut self, remove_files: bool) -> DitResult<()> {
        if remove_files {
            for path in self.stage.files.values() {
                if path.is_file() {
                    remove_file(path)?;
                }
            }
        }
        self.stage.files.clear();
        self.update_stage_file()?;
        Ok(())
    }
}