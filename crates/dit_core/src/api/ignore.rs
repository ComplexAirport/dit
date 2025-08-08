use crate::Dit;
use crate::api_models::ignored_patterns::IgnoredPatterns;
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

/// Getters
impl Dit {
    pub fn get_ignored_list(&self) -> DitResult<IgnoredPatterns> {
        let ignored_patterns = self.ignore_mgr
            .borrow()
            .get_ignored_patterns()?;

        Ok(IgnoredPatterns::from(ignored_patterns))
    }
}
