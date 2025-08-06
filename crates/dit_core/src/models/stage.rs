use std::collections::BTreeMap;
use std::path::PathBuf;
use serde::{Deserialize, Serialize};

/// This struct represents the stage.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Stage {
    /// Maps relative paths of staged files to their blob hashes
    pub files: BTreeMap<PathBuf, PathBuf>,
}
