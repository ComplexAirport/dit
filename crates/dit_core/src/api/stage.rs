use crate::Dit;
use crate::errors::DitResult;
use std::path::{Path, PathBuf};

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
    /// Returns the staged files
    pub fn get_staged_files(&self) -> Vec<PathBuf> {
        self.stage_mgr.borrow()
            .get_stage()
            .files
            .keys()
            .cloned()
            .collect()
    }

    // /// Returns a list of staged files which were modified
    // pub fn get_staged_modified_files(&self) -> DitResult<Vec<PathBuf>> {
    //     let stage_mgr = self.stage_mgr.borrow();
    //     let stage = stage_mgr.get_stage();
    //
    //     let modified = Vec::new();
    //     for (rel_path, blob_hash) in &stage.files {
    //         let abs_path = self.repo.get_absolute_path(rel_path)?;
    //
    //         if abs_path.is_file() {
    //
    //         }
    //     }
    //
    //     Ok(modified)
    // }
}
