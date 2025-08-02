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

use crate::repo::Repo;
use crate::helpers::{
    get_buf_reader,
    read_to_string,
    remove_file,
    write_to_file,
    copy_with_hash_as_name,
};
use crate::errors::{DitResult, StagingError};
use crate::models::Stage;
use std::path::Path;
use std::rc::Rc;

/// Manages the staged files. See [`crate::stage`] for more info
pub struct StageMgr {
    repo: Rc<Repo>,

    stage: Stage,
}

impl StageMgr {
    pub fn from(repo: Rc<Repo>) -> DitResult<Self> {
        let mut mgr = Self {
            repo,
            stage: Stage::new(),
        };
        Self::load_stage_file(&mut mgr)?;
        Ok(mgr)
    }

    pub fn stage(&self) -> &Stage {
        &self.stage
    }
}

impl StageMgr {
    /// Stages a file based on its path. IMPORTANT: files in the
    /// stage folder have their names as their content hashes.
    /// This way, the blob hash doesn't have to be recomputed
    /// when the file is commited
    pub fn stage_file<P: AsRef<Path>>(&mut self, file_path: P) -> DitResult<()> {
        let file_path = self.repo.get_relative_path(file_path)?;
        let reader = get_buf_reader(&file_path)?;

        let hash = copy_with_hash_as_name(reader, self.repo.stage())?;
        let staged_file_path = self.repo.stage().join(&hash);

        self.stage
            .files
            .insert(file_path.to_path_buf(), staged_file_path);

        self.update_stage_file()?;

        Ok(())
    }

    /// Unstages a file based on its path
    pub fn unstage_file<P: AsRef<Path>>(&mut self, file_path: P) -> DitResult<()> {
        let file_path = file_path.as_ref();
        let relative_path = self.repo.get_relative_path(file_path)?;

        let staged_path = self.stage.files.remove(&relative_path);

        if let Some(staged_path) = staged_path {
            remove_file(&staged_path)?;
        }

        self.update_stage_file()?;

        Ok(())
    }

    /// Clears all staged files and clears the [`STAGE_FILE`]
    ///
    /// [`STAGE_FILE`]: crate::project_structure::STAGE_FILE
    pub fn clear_stage(&mut self) -> DitResult<()> {
        for path in self.stage.files.values() {
            remove_file(path)?;
        }
        self.stage.files.clear();
        self.update_stage_file()?;
        Ok(())
    }

    /// Checks whether the stage is empty
    pub fn is_staged(&self) -> bool {
        !self.stage.files.is_empty()
    }

    /// Updates staged files stored in self based on the data in the [`STAGE_FILE`]
    ///
    /// [`STAGE_FILE`]: crate::project_structure::STAGE_FILE
    fn load_stage_file(&mut self) -> DitResult<()> {
        let path = self.repo.stage_file();
        let serialized = read_to_string(path)?;

        let staged_files = if serialized.is_empty() {
            Stage::new()
        } else {
            serde_json::from_str(&serialized)
                .map_err(|_| StagingError::DeserializationError)?
        };

        self.stage = staged_files;

        Ok(())
    }

    /// Updates the data in the [`STAGE_FILE`] based on staged files stored in self
    ///
    /// [`STAGE_FILE`]: crate::project_structure::STAGE_FILE
    fn update_stage_file(&self) -> DitResult<()> {
        let path = self.repo.stage_file();

        let serialized = serde_json::to_string_pretty(&self.stage)
            .map_err(|_| StagingError::SerializationError)?;

        write_to_file(path, serialized)?;

        Ok(())
    }
}
