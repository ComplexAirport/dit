use thiserror::Error;

/// General filesystem related errors
#[derive(Error, Debug)]
pub enum FsError {
    #[error("File '{0}' not found")]
    FileNotFoundError(String),

    #[error("Directory '{0}' not found")]
    DirNotFoundError(String),

    #[error("Failed to create the directory(s) '{0}'")]
    DirCreateError(String),

    #[error("Failed to read the directory '{0}'")]
    DirReadError(String),

    #[error("Failed to remove the directory '{0}'")]
    DirRemoveError(String),

    #[error("Failed to walk the directory '{0}'")]
    DirWalkError(String),

    #[error("Failed to read from the file '{0}'")]
    FileReadError(String),

    #[error("Failed to open the file '{0}'")]
    FileOpenError(String),

    #[error("Failed to write to the file '{0}'")]
    FileWriteError(String),

    #[error("Failed to remove the file '{0}'")]
    FileRemoveError(String),

    #[error("Failed to create the file '{0}'")]
    FileCreateError(String),

    #[error("Failed to rename the file '{0}' to '{1}'")]
    FileRenameError(String, String),

    #[error("Failed to copy the file '{0}' to '{1}'")]
    FileCopyError(String, String),

    #[error("Failed to resolve the metadata for the file '{0}'")]
    FileMetadataResolveError(String),

    #[error("Could not resolve the absolute path for '{0}'")]
    AbsPathResolveError(String),

    #[error("Failed to resolve the current working directory")]
    GetCwdError,

    #[error("Failed to expand the glob pattern '{0}'")]
    GlobPatternError(String),

    #[error("{0}")]
    Other(String),
}
