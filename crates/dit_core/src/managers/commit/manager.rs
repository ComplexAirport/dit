//! This module manages the commits in the Dit version control system
//!
//! A commit is a snapshot of a project's files at a given point in time,
//! along with metadata describing the relative to the last commit.
//! The metadata includes the author who wrote the changes, the parent
//! commit, the commit message, etc.

use crate::Repo;
use std::sync::Arc;

/// Manages the commits in our Dit version control system
pub struct CommitMgr {
    pub(super) repo: Arc<Repo>,
}

/// Constructors
impl CommitMgr {
    pub fn from(repo: Arc<Repo>) -> Self {
        Self { repo }
    }
}
