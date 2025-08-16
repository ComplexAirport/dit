use crate::managers::ignore::manager::{IgnoreMgr, DEFAULT_IGNORE_LIST};
use crate::helpers::{path_to_string, remove_dir, remove_file_if_exists};
use crate::errors::DitResult;
use std::path::{Path, PathBuf};
use ignore::gitignore::Gitignore;
use jwalk::WalkDir;


impl IgnoreMgr {
    /// Walks a specified directory (all the files except the ignored ones)
    /// and applies the given predicate to each file
    pub fn walk_dir_files<P, F>(&self, root: P, mut predicate: F) -> DitResult<()>
    where
        P: AsRef<Path>,
        F: FnMut(PathBuf) -> DitResult<()>
    {
        let root = root.as_ref();
        let ignore = self.ignore.clone();

        WalkDir::new(root)
            .process_read_dir(move |_depth, _path, _state, children| {
                children.retain(|child| {
                    if let Ok(dir_entry) = child {
                        !Self::is_ignored_inner(&dir_entry.path(), &ignore, dir_entry.file_type().is_dir())
                    } else {
                        true
                    }
                });
            })
            .skip_hidden(false)
            .into_iter()
            .filter_map(|r| r.ok())
            .filter(|e| e.depth() != 0 && e.file_type().is_file())
            .try_for_each(|e| {
                predicate(e.path())
            })?;

        Ok(())
    }

    /// Clears a given directory (except the ignored files)
    pub fn clear_dir<P: AsRef<Path>>(&self, root: P) -> DitResult<()> {
        let ignore = self.ignore.clone();
        let mut to_delete: Vec<(PathBuf, bool /* is_dir */)> = WalkDir::new(root)
            .process_read_dir(move |_depth, _path, _state, children| {
                children.retain(|child| match child {
                    Ok(entry) => {
                        if entry.file_type.is_dir() {
                            Self::is_ignored_inner(&entry.path(), &ignore, true)
                        } else {
                            true
                        }
                    }
                    Err(_) => true
                })
            })
            .into_iter()
            .filter_map(Result::ok)
            .filter(|e| e.depth() != 0)
            .map(|e| (e.path(), e.file_type().is_dir()))
            .collect();

        // Sort the deepest entries first so we remove files, then empty directories
        to_delete.sort_by_key(|(p, _)| std::cmp::Reverse(p.components().count()));

        for (path, is_dir) in to_delete {
            if is_dir {
                remove_dir(&path)?;
            } else {
                remove_file_if_exists(&path)?;
            }
        }

        Ok(())
    }

    pub fn is_ignored(&self, path: &Path) -> bool {
        Self::is_ignored_inner(path, &self.ignore, path.is_dir())
    }

    /// Checks if a path is ignored given the [`Gitignore`]
    fn is_ignored_inner(rel_path: &Path, ignore: &Gitignore, is_dir: bool) -> bool {
        ignore.matched_path_or_any_parents(rel_path, is_dir).is_ignore()
        || DEFAULT_IGNORE_LIST.contains(&path_to_string(rel_path).as_str())
    }
}

