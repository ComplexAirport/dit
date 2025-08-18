use thiserror::Error;

/// General filesystem related errors
#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("Failed to serialize the index file")]
    SerializationError,

    #[error("Failed to deserialize the index file")]
    DeserializationError,

    #[error("Configuration required but not found: '{0}'")]
    ConfigNotFound(String),
}
