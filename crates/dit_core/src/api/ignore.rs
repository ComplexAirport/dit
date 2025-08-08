use crate::Dit;
use crate::errors::DitResult;

impl Dit {
    /// Adds all paths from the given glob pattern to the ignore list
    pub fn ignore<S: AsRef<str>>(&self, glob_pat: S) -> DitResult<()> {
        let glob_pat = glob_pat.as_ref();
        self.ignore_mgr.borrow_mut().add_ignore(glob_pat)?;
        Ok(())
    }

    /// Removes all paths from the given glob pattern from the ignore list
    pub fn unignore<S: AsRef<str>>(&self, glob_pat: S) -> DitResult<()> {
        let glob_pat = glob_pat.as_ref();
        self.ignore_mgr.borrow_mut().remove_ignore(glob_pat)?;
        Ok(())
    }
}