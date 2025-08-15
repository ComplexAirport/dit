use thiserror::Error;

/// Errors related to staging
#[derive(Error, Debug)]
pub enum StagingError {
    #[error("Failed to serialize the stage file")]
    SerializationError,

    #[error("Failed to deserialize the stage file")]
    DeserializationError,

    #[error("File not found on the filesystem and is not tracked: '{0}'")]
    FileNotFound(String),
}
