use std::collections::BTreeMap;
use std::path::PathBuf;
use serde::{Deserialize, Serialize};

/// This struct represents the stage.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Stage {
    /// Maps relative paths of staged files to their blob hashes
    ///
    /// NOTE: the [`ChangeType`] here cannot be [`ChangeType::Unchanged`]
    pub files: BTreeMap<PathBuf, ChangeType>,
}


/// This enum represents a change type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ChangeType {
    New(NewFile),
    Modified(ModifiedFile),
    Deleted,
    Unchanged,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewFile {
    pub hash: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModifiedFile {
    pub old_hash: String,
    pub new_hash: String,
}
