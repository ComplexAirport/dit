use thiserror::Error;

/// Errors related to staging
#[derive(Error, Debug)]
pub enum IndexError {
    #[error("Failed to serialize the index file")]
    SerializationError,

    #[error("Failed to deserialize the index file")]
    DeserializationError,

    #[error("File not found on the filesystem and is not tracked: '{0}'")]
    FileNotFound(String),
}
