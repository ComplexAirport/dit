use crate::constants::*;
use std::{fs, io};
use std::io::Error;
use std::path::{Path, PathBuf};

/// Stores paths of the components of the dit project
pub struct DitProject {
    /// Represents the path of the project (where `.dit` is located)
    project_path: PathBuf,

    /// [`DIT_ROOT`]
    dit: PathBuf,

    /// [`BLOBS_ROOT`]
    blobs: PathBuf,

    /// [`TREES_ROOT`]
    trees: PathBuf,

    /// [`STAGE_ROOT`]
    stage: PathBuf,

    /// [`STAGE_FILE`]
    stage_file: PathBuf,

    /// [`COMMITS_ROOT`]
    commits: PathBuf,

    /// [`HEAD_FILE`]
    head_file: PathBuf,
}

/// Constructor
impl DitProject {
    /// Ensures all dir project components are created
    pub fn init<P: AsRef<Path>>(project_path: P) -> io::Result<Self> {
        let project_path = project_path.as_ref().to_path_buf();
        if !project_path.is_dir() {
            return Err(Error::new(
                io::ErrorKind::NotADirectory,
                format!(
                    "given project path '{}' is not a directory",
                    project_path.display()
                ),
            ));
        }

        let dit_root = project_path.join(DIT_ROOT);
        Self::init_sub_dir(&dit_root)?;

        let blobs_root = project_path.join(BLOBS_ROOT);
        Self::init_sub_dir(&blobs_root)?;

        let trees_root = project_path.join(TREES_ROOT);
        Self::init_sub_dir(&trees_root)?;

        let stage_root = project_path.join(STAGE_ROOT);
        Self::init_sub_dir(&stage_root)?;

        let commits_root = project_path.join(COMMITS_ROOT);
        Self::init_sub_dir(&commits_root)?;

        // Initialize the files ONLY after creating all subdirectories
        let stage_file = project_path.join(STAGE_FILE);
        Self::init_sub_file(&stage_file)?;

        let head_file = project_path.join(HEAD_FILE);
        Self::init_sub_file(&head_file)?;

        Ok(Self {
            project_path,
            dit: dit_root,
            blobs: blobs_root,
            trees: trees_root,
            stage: stage_root,
            stage_file,
            commits: commits_root,
            head_file,
        })
    }
    fn init_sub_dir(path: &Path) -> io::Result<()> {
        if !path.is_dir() {
            fs::create_dir_all(path)?;
        }
        Ok(())
    }

    fn init_sub_file(path: &Path) -> io::Result<()> {
        if !path.is_file() {
            // this should not fail because subdirectories are created
            // before creating the files
            fs::File::create(path)?;
        }
        Ok(())
    }
}

/// Getters
impl DitProject {
    /// Returns the project path where the `.dit` is located
    pub fn project_path(&self) -> &Path {
        &self.project_path
    }

    /// Returns the [`DIT_ROOT`] path
    pub fn dit(&self) -> &Path {
        &self.dit
    }

    /// Returns the [`BLOBS_ROOT`] path
    pub fn blobs(&self) -> &Path {
        &self.blobs
    }

    /// Returns the [`TREES_ROOT`] path
    pub fn trees(&self) -> &Path {
        &self.trees
    }

    /// Returns the [`STAGE_ROOT`] path
    pub fn stage(&self) -> &Path {
        &self.stage
    }

    /// Returns the [`STAGE_FILE`] path
    pub fn stage_file(&self) -> &Path {
        &self.stage_file
    }

    /// Returns the [`COMMITS_ROOT`] path
    pub fn commits(&self) -> &Path {
        &self.commits
    }

    /// Returns the [`HEAD_FILE`] path
    pub fn head_file(&self) -> &Path {
        &self.head_file
    }

    /// Returns the path of a given path relative to the project path. \
    /// For example, if the project path is `D:\Projects\project1\` and the given path is
    /// `D:\Projects\project1\src\main.py`, this method will return `src\main.py`
    pub fn get_relative_path<P: AsRef<Path>>(&self, path: P) -> io::Result<PathBuf> {
        let path = path.as_ref();
        if self.includes_path(path) {
            Ok(path
                .strip_prefix(self.project_path())
                .unwrap()
                .to_path_buf())
        } else {
            Err(Error::new(
                io::ErrorKind::InvalidInput,
                format!(
                    "the path '{}' is not a inside the dit project",
                    path.display()
                ),
            ))
        }
    }

    /// Checks whether a given path is inside the project
    pub fn includes_path<P: AsRef<Path>>(&self, path: P) -> bool {
        let path = path.as_ref();
        if !path.exists() {
            return false;
        }
        let abs_project_path = resolve_absolute_path(self.project_path()).unwrap();
        let abs_path = resolve_absolute_path(path).unwrap();
        abs_path.starts_with(abs_project_path)
    }
}

/// Resolves a given path to an absolute, canonical path.
///
/// - Follows symbolic links.
/// - Returns an error if the path does not exist.
/// - On Windows, strips extended-length path prefix (e.g. `\\?\C:\...`)
pub fn resolve_absolute_path(input: &Path) -> io::Result<PathBuf> {
    let canonical = fs::canonicalize(input)?;
    Ok(normalize_path(canonical))
}

#[cfg(windows)]
fn normalize_path(p: PathBuf) -> PathBuf {
    // Remove extended-length prefix like \\?\C:\...
    if let Ok(s) = p.into_os_string().into_string() {
        let cleaned = s.strip_prefix(r"\\?\").unwrap_or(&s);
        PathBuf::from(cleaned)
    } else {
        // fallback for non-UTF-8 paths
        PathBuf::new()
    }
}

#[cfg(not(windows))]
fn normalize_path(p: PathBuf) -> PathBuf {
    p
}
