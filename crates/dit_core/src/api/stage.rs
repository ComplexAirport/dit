use crate::Dit;
use crate::errors::DitResult;
use std::path::Path;

/// Manipulate the stage
impl Dit {
    /// Stages a file given the file path
    pub fn stage_files(&mut self, paths: impl IntoIterator<Item = impl AsRef<Path>>) -> DitResult<()> {
        self.stage_mgr()?.borrow_mut().stage_files(
            paths,
            &self.blob_mgr().borrow(),
            &self.tree_mgr().borrow(),
            &self.commit_mgr().borrow(),
            &self.branch_mgr()?.borrow(),
        )
    }

    /// Unstages the file given the file path
    pub fn unstage_files(&mut self, paths: impl IntoIterator<Item = impl AsRef<Path>>) -> DitResult<()> {
        self.stage_mgr()?.borrow_mut().unstage_files(paths)
    }

    /// Clears the stage
    pub fn clear_stage(&mut self) -> DitResult<()> {
        self.stage_mgr()?.borrow_mut().clear_stage_all(
            &self.blob_mgr().borrow()
        )
    }
}
