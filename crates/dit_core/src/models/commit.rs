use crate::errors::{CommitError, DitResult};
use crate::helpers::{read_to_string, write_to_file, path_to_string};
use crate::impl_read_write_model;
use serde::{Deserialize, Serialize};
use std::path::Path;

/// Represents a commit model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Commit {
    /// Represents the committer name and email address \
    /// Example: "Alice | alice@example.com"
    pub author: String,

    /// Represents the commit message \
    /// Example: "initial commit"
    pub message: String,

    /// Represents the commit time as a Unix timestamp - number of seconds
    /// since January 1, 1970 (UTC)
    pub timestamp: u64,

    /// Represents the tree hash of this commit
    pub tree: String,

    /// Represents the hash (or hashes) of the parent commit(s)
    pub parents: Vec<String>,

    /// Represents the commit hash
    #[serde(skip)]
    pub hash: String,
}

impl_read_write_model!(Commit, CommitError);
