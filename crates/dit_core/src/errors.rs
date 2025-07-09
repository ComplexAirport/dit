use thiserror::Error;

#[derive(Error, Debug)]
pub enum DitCoreError {
    #[error("staging error: {0}")]
    StagingError(#[from] StagingError),

    #[error("commit error: {0}")]
    CommitError(#[from] CommitError),

    #[error("tree error: {0}")]
    TreeError(#[from] TreeError),

    #[error("blob error: {0}")]
    BlobError(#[from] BlobError),

    #[error("project error: {0}")]
    ProjectError(#[from] ProjectError),

    #[error("filesystem error: {0}")]
    FsError(#[from] FsError),

    #[error("error: {0}")]
    OtherError(#[from] OtherError),
}

pub type DitResult<T> = Result<T, DitCoreError>;


/// Errors related to staging
#[derive(Error, Debug)]
pub enum StagingError {
    #[error("Failed to serialize the stage file")]
    SerializationError,

    #[error("Failed to deserialize the stage file")]
    DeserializationError,
}


/// Errors related to commits
#[derive(Error, Debug)]
pub enum CommitError {
    #[error("Failed to serialize the commit with hash '{0}'")]
    SerializationError(String),

    #[error("Failed to deserialize the commit with hash '{0}'")]
    DeserializationError(String),
}


/// Errors related to trees
#[derive(Error, Debug)]
pub enum TreeError {
    #[error("Failed to serialize the tree with hash '{0}'")]
    SerializationError(String),

    #[error("Failed to deserialize the tree with hash '{0}'")]
    DeserializationError(String),
}


/// Errors related to blobs
#[derive(Error, Debug)]
pub enum BlobError {
    #[error("Failed to write the blob '{0}'")]
    BlobWriteError(String),

    #[error("Failed to read the blob '{0}'")]
    BlobReadError(String),

    #[error("Failed to create a temporary blob file '{0}'")]
    TempFileCreationError(String),

    #[error("Failed to delete the temporary blob file '{0}'")]
    TempFileDeletionError(String),

    #[error("Failed to rename the temporary blob file '{0}' to '{1}'")]
    TempFileRenameError(String, String),
}


/// General dit project-related errors
#[derive(Error, Debug)]
pub enum ProjectError {
    #[error("The file '{0}' is not inside the project")]
    FileNotInProject(String),

    #[error("The given project path '{0}' is not a directory")]
    ProjectPathNotADirectory(String),

    #[error("Failed to create .dit project subdirectory '{0}'")]
    SubDirCreationError(String),

    #[error("Failed to create .dit project file '{0}'")]
    SubFileCreationError(String),
}


/// General filesystem related errors
#[derive(Error, Debug)]
pub enum FsError {
    #[error("File '{0}' not found")]
    FileNotFoundError(String),

    #[error("Directory '{0}' not found")]
    DirNotFoundError(String),

    #[error("Failed to read from the file '{0}'")]
    FileReadError(String),

    #[error("Failed to open the file '{0}'")]
    FileOpenError(String),

    #[error("Failed to write to the file '{0}'")]
    FileWriteError(String),

    #[error("Failed to remove the file '{0}'")]
    FileRemoveError(String),

    #[error("Could not resolve the absolute path for '{0}'")]
    AbsPathResolveError(String),
}


/// Other errors
#[derive(Error, Debug)]
pub enum OtherError {
    #[error("Current system time is earlier then the unix epoch time.")]
    TimeWentBackwardsError,
}