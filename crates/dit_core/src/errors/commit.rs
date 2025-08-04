use thiserror::Error;

/// Errors related to commits
#[derive(Error, Debug)]
pub enum CommitError {
    #[error("Failed to serialize the commit with hash '{0}'")]
    SerializationError(String),

    #[error("Failed to deserialize the commit with hash '{0}'")]
    DeserializationError(String),

    #[error("The commit '{0}' is unreachable from the commit '{1}'. Use hard reset instead.")]
    UnreachableCommitError(String, String),
}
