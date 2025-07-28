use crate::constants::*;
use crate::errors::{DitResult, ProjectError};
use crate::helpers::resolve_absolute_path;
use std::fs::{self, File};
use std::path::{Path, PathBuf};

/// Stores paths of the components of the dit project
pub struct DitProject {
    /// Represents the path of the project (where `.dit` is located)
    project_path: PathBuf,

    /// [`DIT_ROOT`]
    dit_root: PathBuf,

    /// [`BLOBS_ROOT`]
    blobs_root: PathBuf,

    /// [`TREES_ROOT`]
    trees_root: PathBuf,

    /// [`STAGE_ROOT`]
    stage_root: PathBuf,

    /// [`STAGE_FILE`]
    stage_file: PathBuf,

    /// [`COMMITS_ROOT`]
    commits_root: PathBuf,

    /// [`BRANCHES_ROOT`]
    branches_root: PathBuf,

    /// [`HEAD_FILE`]
    head_file: PathBuf,
}

/// Constructor
impl DitProject {
    /// Ensures all dir project components are created
    pub fn init<P: AsRef<Path>>(project_path: P) -> DitResult<Self> {
        let project_path = project_path.as_ref().to_path_buf();

        if !project_path.is_dir() {
            return Err(ProjectError::ProjectPathNotADirectory(
                project_path.display().to_string()).into());
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

        let branches_root = project_path.join(BRANCHES_ROOT);
        Self::init_sub_dir(&branches_root)?;

        // Initialize the files ONLY after creating all subdirectories
        let stage_file = project_path.join(STAGE_FILE);
        Self::init_sub_file(&stage_file)?;

        let head_file = project_path.join(HEAD_FILE);
        Self::init_sub_file(&head_file)?;

        Ok(Self {
            project_path,
            dit_root,
            blobs_root,
            trees_root,
            stage_root,
            stage_file,
            commits_root,
            branches_root,
            head_file,
        })
    }

    fn init_sub_dir(path: &Path) -> DitResult<()> {
        if !path.is_dir() {
            fs::create_dir_all(path)
                .map_err(|_|
                    ProjectError::SubDirCreationError(path.display().to_string()))?;

        }
        Ok(())
    }

    fn init_sub_file(path: &Path) -> DitResult<()> {
        if !path.is_file() {
            // this should not fail because subdirectories are created
            // before creating the files
            File::create(path)
                .map_err(|_|
                    ProjectError::SubFileCreationError(path.display().to_string())
                )?;
        }
        Ok(())
    }
}

/// API
impl DitProject {
    /// Returns the project path where the `.dit` is located
    pub fn project_path(&self) -> &Path {
        &self.project_path
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

    /// Returns the [`STAGE_ROOT`] path
    pub fn stage(&self) -> &Path {
        &self.stage_root
    }

    /// Returns the [`STAGE_FILE`] path
    pub fn stage_file(&self) -> &Path {
        &self.stage_file
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

    /// Returns the absolute path of a given relative path in the project
    pub fn get_absolute_path<P: AsRef<Path>>(&self, relative_path: P) -> DitResult<PathBuf> {
        let path = relative_path.as_ref();
        if self.includes_path(path) {
            Ok(resolve_absolute_path(path)?)
        } else {
            Err(ProjectError::FileNotInProject(path.display().to_string()).into())
        }
    }

    /// Returns the path of a given path relative to the project path. \
    /// For example, if the project path is `D:\Projects\project1\` and the given path is
    /// `D:\Projects\project1\src\main.py`, this method will return `src\main.py`
    pub fn get_relative_path<P: AsRef<Path>>(&self, path: P) -> DitResult<PathBuf> {
        let path = path.as_ref();
        if self.includes_path(path) {
            Ok(path
                .strip_prefix(self.project_path())
                .unwrap()
                .to_path_buf())
        } else {
            Err(ProjectError::FileNotInProject(path.display().to_string()).into())
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
