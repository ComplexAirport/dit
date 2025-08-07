//! This module manages the trees in dit version control system
//!
//! Trees are *snapshots* of all files included in the repo during a particular commit.
//! They store all files and their corresponding blob hashes. Each commit has one
//! corresponding tree.
//!
//! Let's say we do an initial commit with only one file, `a.txt`.
//! We can imagine the tree for this commit as something like this (simplified): \
//! `.dit/trees/18b7cb09..`
//! ```json
//! {
//!     "files": {
//!         "a.txt": "b1ac8a822.."
//!     }
//! }
//! ```
//!
//! Suppose in the next commit we commited a single new file, `b.txt` and did not change `a.txt`
//! The tree for this commit will look like this: \
//! `.dit/trees/c52b214f..`
//! ```json
//! {
//!     "files": {
//!         "a.txt": "b1ac8a82..",
//!         "b.txt": "273c662e.."
//!     }
//! }
//! ```

use crate::Repo;
use std::sync::Arc;

/// Manages the trees in our Dit version control system
pub struct TreeMgr {
    pub(super) repo: Arc<Repo>,
}

/// Constructors
impl TreeMgr {
    pub fn from(repo: Arc<Repo>) -> Self {
        Self { repo }
    }
}
