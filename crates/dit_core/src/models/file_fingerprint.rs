use crate::helpers::path_to_string;
use crate::errors::{DitResult, FsError};
use std::path::Path;
use std::time::SystemTime;
use serde::{Deserialize, Serialize};

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
