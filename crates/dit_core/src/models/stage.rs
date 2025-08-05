use std::collections::HashMap;
use std::path::PathBuf;
use serde::{Deserialize, Serialize};

/// This struct represents the stage.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Stage {
    /// Maps relative paths of staged files to their blob hashes
    pub files: HashMap<PathBuf, PathBuf>,
}

impl Stage {
    pub fn new() -> Self {
        Self {
            files: HashMap::new(),
        }
    }
}
