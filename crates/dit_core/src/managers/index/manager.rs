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
//!         "src/main.py": "D:\test_project\.dit\index\main.py"
//!     }
//! }
//! ```
//! This file maps real (and relative) file locations in the project to the
//! location of the copied file in the temporary "buffer" zone. This way,
//! when a commit happens, the system knows where to find the staged file content.

use crate::Repo;
use crate::errors::DitResult;
use crate::models::Index;
use std::sync::Arc;

/// Manages the staged files. See [`crate::index`] for more info
pub struct IndexMgr {
    pub(super) repo: Arc<Repo>,

    pub(super) index: Index,
}


impl IndexMgr {
    pub fn from(repo: Arc<Repo>) -> DitResult<Self> {
        let mut mgr = Self {
            repo,
            index: Index::default(),
        };
        Self::load(&mut mgr)?;
        Ok(mgr)
    }
}

