use crate::Dit;
use crate::errors::DitResult;
use std::path::Path;

/// Manipulate the index
impl Dit {
    /// Adds files in their current state to the index
    pub fn add_files(&mut self, paths: impl IntoIterator<Item = impl AsRef<Path>>) -> DitResult<()> {
        self.index_mgr()?.borrow_mut().add_files(
            paths,
            &self.blob_mgr().borrow(),
            &self.tree_mgr().borrow(),
            &self.commit_mgr().borrow(),
            &self.branch_mgr()?.borrow(),
        )
    }

    /// Unstages files
    pub fn unstage_files(&mut self, paths: impl IntoIterator<Item = impl AsRef<Path>>) -> DitResult<()> {
        self.index_mgr()?.borrow_mut().unstage_files(
            paths,
            &self.tree_mgr().borrow(),
            &self.commit_mgr().borrow(),
            &self.branch_mgr()?.borrow(),
        )
    }

    /// Clears the index
    pub fn clear_stage(&mut self) -> DitResult<()> {
        self.index_mgr()?.borrow_mut().unstage_all(
            &self.tree_mgr().borrow(),
            &self.commit_mgr().borrow(),
            &self.branch_mgr()?.borrow(),
        )
    }
}
