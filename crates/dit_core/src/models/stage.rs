use crate::errors::{DitResult, FsError};
use crate::helpers::path_to_string;
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::time::SystemTime;
use serde::{Deserialize, Serialize};

/// This struct represents the stage.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Stage {
    /// Maps relative paths of tracked files to their blob hashes
    pub files: BTreeMap<PathBuf, ChangeType>,
}


/// This enum represents a change type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ChangeType {
    New(NewFile),
    Modified(ModifiedFile),
    Deleted,
    Unchanged(UnchangedFile),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewFile {
    /// Hash of the file's content
    pub hash: String,

    /// File fingerprint
    pub fingerprint: FileFingerprint,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModifiedFile {
    /// Old hash of the file's content
    pub old_hash: String,

    /// New hash of the file's content
    pub hash: String,

    /// File fingerprint
    pub fingerprint: FileFingerprint,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnchangedFile {
    /// Hash of the file's content
    pub hash: String,

    /// File fingerprint
    pub fingerprint: FileFingerprint,
}

#[derive(Debug, Clone, Serialize, Deserialize, Ord, PartialOrd, Eq, PartialEq)]
pub struct FileFingerprint {
    /// File size in bytes
    pub size: u64,

    /// Modification time
    pub modified_at: SystemTime,
}


impl FileFingerprint {
    pub fn from(path: &Path) -> DitResult<Self> {
        let metadata = std::fs::symlink_metadata(path)
            .map_err(|_| FsError::FileMetadataResolveError(path_to_string(path)))?;

        let size = metadata.len();
        let modified_at = metadata.modified()
            .map_err(|_| FsError::FileMetadataResolveError(path_to_string(path)))?;

        Ok(Self { size, modified_at })
    }
}
