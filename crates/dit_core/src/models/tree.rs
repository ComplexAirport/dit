use crate::models::Index;
use serde::{Deserialize, Serialize};

/// Represents a tree model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tree {
    /// Maps the relative file paths to corresponding blob hashes
    pub index: Index,

    /// Represents the tree hash
    #[serde(skip)]
    pub hash: String,
}
