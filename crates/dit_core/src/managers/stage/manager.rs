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
use crate::errors::DitResult;
use crate::models::Stage;
use std::rc::Rc;

/// Manages the staged files. See [`crate::stage`] for more info
pub struct StageMgr {
    pub(super) repo: Rc<Repo>,

    pub(super) stage: Stage,
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
}

