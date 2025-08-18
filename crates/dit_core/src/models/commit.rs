use serde::{Deserialize, Serialize};

/// Represents a commit model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Commit {
    /// Represents the committer name and email address \
    /// Example: "Alice <alice@example.com>"
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
