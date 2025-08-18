use crate::Dit;
use crate::errors::DitResult;
use std::path::PathBuf;

impl Dit {
    /// Expands the given glob patterns to file paths relative to the
    /// current working directory
    pub fn expand_globs_cwd<I>(&self, globs: I) -> DitResult<Vec<PathBuf>>
    where I: Iterator,
          I::Item: AsRef<str>
    {
        self.ignore_mgr()?.borrow().expand_globs_cwd(globs)
    }
}