use crate::managers::ignore::manager::{IgnoreMgr, DEFAULT_IGNORE_LIST};
use crate::helpers::{path_to_string, remove_dir, remove_file_if_exists};
use crate::errors::DitResult;
use std::path::{Path, PathBuf};
use jwalk::WalkDir;


impl IgnoreMgr {
    pub fn is_ignored<P: AsRef<Path>>(&self, path: P) -> bool {
        _is_ignored(path, &self.ignored_list)
    }

    /// Walks a specified directory (all the files except the ignored ones)
    /// and applies the given predicate to each file
    pub fn walk_dir_files<P, F>(&self, root: P, mut predicate: F) -> DitResult<()>
    where
        P: AsRef<Path>,
        F: FnMut(PathBuf) -> DitResult<()>
    {
        let root = root.as_ref();
        let ignored_list = self.ignored_list.clone();

        WalkDir::new(root)
            .process_read_dir(move |_depth, _path, _state, children| {
                children.retain(|child| {
                    if let Ok(dir_entry) = child {
                        !_is_ignored(dir_entry.path(), &ignored_list)
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
        let ignored_list = self.ignored_list.clone();
        let mut to_delete: Vec<(PathBuf, bool /* is_dir */)> = WalkDir::new(root)
            .process_read_dir(move |_depth, _path, _state, children| {
                children.retain(|child| match child {
                    Ok(entry) => {
                        if entry.file_type.is_dir() {
                            !_is_ignored(entry.path(), &ignored_list)
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
}


fn _is_ignored<P1: AsRef<Path>>(rel_path: P1, ignore_list: &[PathBuf]) -> bool
{
    let path = rel_path.as_ref();

    let in_ignore = path
        .ancestors()
        .map(|p| p.to_path_buf())
        .any(|ancestor| ignore_list.contains(&ancestor));

    let in_default_ignore = DEFAULT_IGNORE_LIST
        .contains(&path_to_string(path).as_str());

    in_ignore || in_default_ignore
}
