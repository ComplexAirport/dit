use crate::Dit;
use crate::errors::DitResult;
use std::path::Path;

/// Manipulate the stage
impl Dit {
    /// Stages a file given the file path
    pub fn stage_file(&mut self, path: &Path) -> DitResult<()> {
        self.stage_mgr()?.borrow_mut().stage_file(
            &path,
            &self.blob_mgr().borrow(),
            &self.tree_mgr().borrow(),
            &self.commit_mgr().borrow(),
            &self.branch_mgr()?.borrow(),
        )
    }

    /// Unstages the file given the file path
    pub fn unstage_file(&mut self, path: &Path) -> DitResult<()> {
        self.stage_mgr()?.borrow_mut().unstage_file(path)
    }

    /// Clears the stage
    pub fn clear_stage(&mut self) -> DitResult<()> {
        self.stage_mgr()?.borrow_mut().clear_stage_all(
            &self.blob_mgr().borrow()
        )
    }
}
