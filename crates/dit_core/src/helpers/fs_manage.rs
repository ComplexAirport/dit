use crate::helpers::resolve_absolute_path;
use crate::errors::{DitResult, FsError};
use std::collections::HashSet;
use std::fs;
use std::fs::File;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

/// Removes a file using [`fs::remove_file`] and maps the error to [`FsError`]
pub fn remove_file<P: AsRef<Path>>(path: P) -> DitResult<()> {
    let path = path.as_ref();
    fs::remove_file(path)
        .map_err(|_| FsError::FileRemoveError(path.display().to_string()).into())
}


/// Removes a directory using [`fs::remove_dir`] and maps the error to [`FsError`]
pub fn remove_dir<P: AsRef<Path>>(path: P) -> DitResult<()> {
    let path = path.as_ref();
    fs::remove_dir_all(path)
        .map_err(|_| FsError::DirRemoveError(path.display().to_string()).into())
}

/// Removes a directory using [`fs::remove_dir_all`] and maps the error to [`FsError`]
pub fn remove_dir_all<P: AsRef<Path>>(path: P) -> DitResult<()> {
    let path = path.as_ref();
    fs::remove_dir_all(path)
        .map_err(|_| FsError::DirRemoveError(path.display().to_string()).into())
}


/// Creates a file and all the necessary subdirectories (if they don't exist) and maps
/// the result to [`FsError`]
pub fn create_file_all<P: AsRef<Path>>(path: P) -> DitResult<()> {
    let path = path.as_ref();

    if path.is_file() {
        return Ok(());
    }

    if let Some(parent) = Path::new(path).parent() {
        fs::create_dir_all(parent)
            .map_err(|_| FsError::DirCreateError(parent.display().to_string()))?;
    }

    File::create(path)
        .map_err(|_| FsError::FileCreateError(path.display().to_string()))?;

    Ok(())
}


/// Deletes every file / directory inside `root` except the paths in `keep`.
pub fn clear_dir_except<P, I>(root: P, keep: I) -> DitResult<()>
where
    P: AsRef<Path>,
    I: IntoIterator,
    I::Item: AsRef<Path>,
{
    let root = resolve_absolute_path(root.as_ref())?;

    let mut keep_set: HashSet<PathBuf> = HashSet::new();
    for k in keep {
        let kp = root.join(k.as_ref());
        keep_set.insert(resolve_absolute_path(&kp)?);
    }

    // Helper: does `p` start with any kept prefix?
    let is_kept = |p: &Path| keep_set.iter().any(|k| p.starts_with(k));

    for entry in WalkDir::new(&root).min_depth(1).contents_first(true) {
        let entry = entry
            .map_err(|_| FsError::DirWalkError(root.display().to_string()))?;

        let path = entry.path();

        if is_kept(path) {
            continue;
        }

        if entry.file_type().is_dir() {
            remove_dir(path)?;
        } else {
            remove_file(path)?;
        }
    }

    Ok(())
}
