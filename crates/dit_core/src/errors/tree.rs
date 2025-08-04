use thiserror::Error;

/// Errors related to trees
#[derive(Error, Debug)]
pub enum TreeError {
    #[error("Failed to serialize the tree with hash '{0}'")]
    SerializationError(String),

    #[error("Failed to deserialize the tree with hash '{0}'")]
    DeserializationError(String),
}
