use crate::Dit;
use crate::errors::DitResult;
use crate::api::models::Status;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;
use crate::helpers::calculate_hash;

/// Manipulate the stage
impl Dit {
    /// Stages a file given the file path
    pub fn stage<P: AsRef<Path>>(&mut self, path: P) -> DitResult<()> {
        self.stage_mgr.borrow_mut().stage_file(path)
    }

    /// Unstages the file given the file path
    pub fn unstage<P: AsRef<Path>>(&mut self, path: P) -> DitResult<()> {
        self.stage_mgr.borrow_mut().unstage_file(path)
    }

    /// Clears the stage
    pub fn clear_stage(&mut self) -> DitResult<()> {
        self.stage_mgr.borrow_mut().clear_stage(true)
    }
}


/// Getters
impl Dit {
    /// Returns the status
    pub fn get_status(&self) -> DitResult<Status> {
        let stage_mgr = self.stage_mgr.borrow();
        let stage = &stage_mgr.get_stage().files;

        let mut status = Status::new();

        for entry in WalkDir::new(self.repo.repo_path())
            .min_depth(1)
            .into_iter()
            .filter_map(Result::ok)
            .filter(|e| e.file_type().is_file())
        {
            let abs_path = entry.path().to_path_buf();
            let rel_path = self.repo.get_relative_path(&abs_path)?;

            if let Some(blob_path) = stage.get(&rel_path) {
                if !abs_path.is_file() {
                    status.add_staged_deleted_file(rel_path);
                    continue;
                }

                let blob_hash = blob_path
                    .file_name()
                    .unwrap_or_default()
                    .to_string_lossy()
                    .to_string();

                let hash = calculate_hash(abs_path)?;
                if hash != blob_hash {
                    status.add_staged_modified_file(rel_path);
                } else {
                    status.add_staged_unchanged_file(rel_path);
                }
            } else {
                status.add_untracked_file(rel_path);
            }
        }

        // todo: iterate through staged_files now

        Ok(status)
    }
}
