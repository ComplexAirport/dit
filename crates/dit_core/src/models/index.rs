use crate::models::file_fingerprint::FileFingerprint;
use std::collections::BTreeMap;
use std::path::PathBuf;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Index {
    /// Maps the relative paths of the files to their corresponding entries
    pub files: BTreeMap<PathBuf, IndexEntry>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndexEntry {
    /// Represents the file hash
    pub hash: String,

    /// Represents the file fingerprint
    pub fp: FileFingerprint,
}

