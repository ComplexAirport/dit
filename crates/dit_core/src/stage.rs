//! This module manages staged files in dit version control system
//!
//! Staged files are the files that are tracked but not committed yet.
//! Image it as a "waiting" zone for files which are to be committed later.
//! When a file is staged, it is copied to a special buffer zone and information
//! about the file is stored in a separate file. \
//! The file looks something like this (simplified):
//! ```json
//! {
//!     "files": {
//!         "src/main.py": "D:\test_project\.dit\stage\main.py"
//!     }
//! }
//! ```
//! This file maps real (and relative) file locations in the project to the
//! location of the copied file in the temporary "buffer" zone. This way,
//! when a commit happens, the system knows where to find the staged file content.

use crate::dit_project::DitProject;
use crate::helpers::{
    get_buf_reader,
    get_buf_writer,
    read_to_string,
    remove_file,
    write_to_file,
    transfer_data,
};
use crate::errors::{DitResult, StagingError};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::rc::Rc;
use uuid::Uuid;

/// Manages the staged files. See [`crate::stage`] for more info
pub struct StageMgr {
    project: Rc<DitProject>,

    staged_files: StagedFiles,
}

impl StageMgr {
    pub fn from(project: Rc<DitProject>) -> DitResult<Self> {
        let mut mgr = Self {
            project,
            staged_files: StagedFiles::new(),
        };
        Self::load_stage_file(&mut mgr)?;
        Ok(mgr)
    }

    pub fn staged_files(&self) -> &StagedFiles {
        &self.staged_files
    }
}

impl StageMgr {
    /// Stages a file based on its path
    pub fn stage_file<P: AsRef<Path>>(&mut self, file_path: P) -> DitResult<()> {
        let file_path = self.project.get_relative_path(file_path)?;

        // generate a unique filename to copy the staged file to
        let write_to = loop {
            let name = format!("file-{}", Uuid::new_v4());
            let path = self.project.stage().join(name);
            if !path.exists() {
                break path;
            }
        };

        let mut reader = get_buf_reader(&file_path)?;
        let mut writer = get_buf_writer(&write_to)?;

        transfer_data(&mut reader, &mut writer, &file_path)?;

        self.staged_files
            .files
            .insert(file_path.to_path_buf(), write_to);

        self.update_stage_file()?;

        Ok(())
    }

    /// Unstages a file based on its path
    pub fn unstage_file<P: AsRef<Path>>(&mut self, file_path: P) -> DitResult<()> {
        let file_path = file_path.as_ref();
        let relative_path = self.project.get_relative_path(file_path)?;

        let staged_path = self.staged_files.files.remove(&relative_path);

        if let Some(staged_path) = staged_path {
            remove_file(&staged_path)?;
        }

        self.update_stage_file()?;

        Ok(())
    }

    /// Clears all staged files and clears the [`STAGE_FILE`]
    ///
    /// [`STAGE_FILE`]: crate::constants::STAGE_FILE
    pub fn clear_stage(&mut self) -> DitResult<()> {
        for path in self.staged_files.files.values() {
            remove_file(path)?;
        }
        self.staged_files.files.clear();
        self.update_stage_file()?;
        Ok(())
    }

    /// Updates staged files stored in self based on the data in the [`STAGE_FILE`]
    ///
    /// [`STAGE_FILE`]: crate::constants::STAGE_FILE
    fn load_stage_file(&mut self) -> DitResult<()> {
        let path = self.project.stage_file();
        let serialized = read_to_string(&path)?;

        let staged_files = if serialized.is_empty() {
            StagedFiles::new()
        } else {
            serde_json::from_str(&serialized)
                .map_err(|_| StagingError::DeserializationError)?
        };

        self.staged_files = staged_files;

        Ok(())
    }

    /// Updates the data in the [`STAGE_FILE`] based on staged files stored in self
    ///
    /// [`STAGE_FILE`]: crate::constants::STAGE_FILE
    fn update_stage_file(&mut self) -> DitResult<()> {
        let path = self.project.stage_file();

        let serialized = serde_json::to_string_pretty(&self.staged_files)
            .map_err(|_| StagingError::SerializationError)?;

        write_to_file(&path, serialized)?;

        Ok(())
    }
}

/// This struct represents staged files. \
/// NOTE: this is later used in [`TreeMgr`] to create trees
///
/// [`TreeMgr`]: crate::tree::TreeMgr
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StagedFiles {
    /// Staged files copied into the staging area and are named using UUID-s
    /// This field maps the real (project-relative) paths to the staged versions
    pub files: HashMap<PathBuf, PathBuf>,
}

impl StagedFiles {
    pub fn new() -> Self {
        Self {
            files: HashMap::new(),
        }
    }
}
