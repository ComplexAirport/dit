use crate::errors::DitResult;
use crate::helpers::expand_glob;
use crate::managers::ignore::IgnoreMgr;

impl IgnoreMgr {
    /// Adds all paths from the given glob pattern to the ignore list
    pub fn add_ignore<S: AsRef<str>>(&mut self, glob_pat: S) -> DitResult<()> {
        let glob_pat = glob_pat.as_ref();
        self.ignored_patterns.push(glob_pat.to_string());
        self.ignored_list.extend(expand_glob(self.repo.repo_path(), glob_pat)?);
        self.store()?;
        Ok(())
    }


    /// Removes all paths from the given glob pattern from the ignore list
    pub fn remove_ignore<S: AsRef<str>>(&mut self, glob_pat: S) -> DitResult<()> {
        let glob_pat = glob_pat.as_ref();
        let paths = expand_glob(self.repo.repo_path(), glob_pat)?;
        self.ignored_patterns.retain(|p| p != glob_pat);
        self.ignored_list.retain(|p| !paths.contains(p));
        self.store()?;
        Ok(())
    }
}
