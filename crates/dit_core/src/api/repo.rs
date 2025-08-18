use crate::errors::{DitResult, ProjectError};
use crate::helpers::{get_cwd, path_to_string, resolve_absolute_path};
use super::dit_component_paths::*;
use std::fs::{self, OpenOptions};
use std::path::{Path, PathBuf};

/// Stores paths of the components of the dit repository
pub struct Repo {
    /// Represents the path of the repository (where `.dit` is located)
    repo_path: PathBuf,
    dit_root: PathBuf,
    blobs_root: PathBuf,
    trees_root: PathBuf,
    index_file: PathBuf,
    commits_root: PathBuf,
    branches_root: PathBuf,
    head_file: PathBuf,
    ignore_file: PathBuf,
    config_file: PathBuf,
}

/// Constructor
impl Repo {
    /// Ensures all .dit components are created
    pub fn init<P: AsRef<Path>>(project_path: P) -> DitResult<Self> {
        let repo_path = resolve_absolute_path(project_path.as_ref())?;

        if !repo_path.is_dir() {
            return Err(ProjectError::ProjectPathNotADirectory(path_to_string(&repo_path)).into());
        }

        /*************************
        * Component Directories
        *************************/
        let dit_root = repo_path.join(DIT_ROOT);
        let blobs_root = repo_path.join(BLOBS_ROOT);
        let trees_root = repo_path.join(TREES_ROOT);
        let commits_root = repo_path.join(COMMITS_ROOT);
        let branches_root = repo_path.join(BRANCHES_ROOT);

        let component_dirs = [
            &dit_root, &blobs_root, &trees_root, &commits_root, &branches_root
        ];

        /*************************
        * Component Files
        *************************/
        let index_file = repo_path.join(INDEX_FILE);
        let head_file = repo_path.join(HEAD_FILE);
        let config_file = repo_path.join(CONFIG_FILE);
        let component_files = [
            &index_file, &head_file, &config_file,
        ];

        for component_dir in component_dirs {
            Self::init_sub_dir(component_dir)?;
        }

        for component_path in component_files {
            Self::init_sub_file(component_path)?;
        }

        let ignore_file = repo_path.join(IGNORE_FILE);
        Ok(Self {
            repo_path, dit_root, blobs_root,
            trees_root, index_file, commits_root,
            branches_root, head_file, ignore_file,
            config_file,
        })
    }

    fn init_sub_dir(path: &Path) -> DitResult<()> {
        match fs::create_dir_all(path) {
            Ok(_) => Ok(()),
            Err(e) if e.kind() == std::io::ErrorKind::AlreadyExists => Ok(()),
            Err(e) => Err(e.into()),
        }
    }

    fn init_sub_file(path: &Path) -> DitResult<()> {
        OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(false)
            .open(path)?;

        Ok(())
    }
}

/// Path getters
impl Repo {
    /// Returns the project path where the `.dit` is located
    pub fn repo_path(&self) -> &Path {
        &self.repo_path
    }

    /// Returns the [`DIT_ROOT`] path
    pub fn dit(&self) -> &Path {
        &self.dit_root
    }

    /// Returns the [`BLOBS_ROOT`] path
    pub fn blobs(&self) -> &Path {
        &self.blobs_root
    }

    /// Returns the [`TREES_ROOT`] path
    pub fn trees(&self) -> &Path {
        &self.trees_root
    }

    /// Returns the [`INDEX_FILE`] path
    pub fn index_file(&self) -> &Path {
        &self.index_file
    }

    /// Returns the [`COMMITS_ROOT`] path
    pub fn commits(&self) -> &Path {
        &self.commits_root
    }

    /// Returns the [`BRANCHES_ROOT`] path
    pub fn branches(&self) -> &Path {
        &self.branches_root
    }

    /// Returns the [`HEAD_FILE`] path
    pub fn head_file(&self) -> &Path {
        &self.head_file
    }

    /// Returns the [`IGNORE_FILE`] path
    pub fn ignore_file(&self) -> &Path {
        &self.ignore_file
    }

    /// Returns the [`CONFIG_FILE`] path
    pub fn config_file(&self) -> &Path {
        &self.config_file
    }

    /// Returns the absolute path of a given path.
    /// 1. If the given path is relative, it will be considered relative to project path
    /// 2. If the given file is absolute, nothing will change
    pub fn abs_path_from_repo(&self, path: &Path, missing_ok: bool) -> DitResult<PathBuf> {
        if path.is_absolute() {
            Ok(path.to_path_buf())
        } else {
            let res = self.repo_path.join(path);
            if missing_ok || res.exists() {
                Ok(res)
            } else  {
                Err(ProjectError::NotInProject(path_to_string(path)).into())
            }
        }
    }

    /// Returns the absolute path of a given path.
    /// 1. If the given path is relative, it will be considered relative to the current working
    ///    directory
    /// 2. If the given file is absolute, nothing will change
    pub fn abs_path_from_cwd(&self, path: &Path, missing_ok: bool) -> DitResult<PathBuf> {
        if path.is_absolute() {
            Ok(path.to_path_buf())
        } else if !missing_ok {
            resolve_absolute_path(path)
        } else {
            Ok(get_cwd()?.join(path))
        }
    }

    /// Returns the relative path (relative to the project path) of a given path
    ///
    /// NOTE: If the given path is relative, it will be considered relative to the
    /// current working directory. Returns an error if the project does not contain such a path
    pub fn rel_path(&self, path: &Path) -> DitResult<PathBuf> {
        let abs_path = self.abs_path_from_cwd(path, false)?;

        match abs_path.strip_prefix(&self.repo_path) {
            Ok(p) => Ok(p.to_path_buf()),
            Err(_) => Err(ProjectError::NotInProject(path_to_string(path)).into())
        }
    }
}
