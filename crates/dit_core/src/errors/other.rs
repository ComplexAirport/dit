use thiserror::Error;

/// Other errors
#[derive(Error, Debug)]
pub enum OtherError {
    #[error("Current system time is earlier then the unix epoch time.")]
    TimeWentBackwardsError,

    #[error("Failed to flush the buffer.")]
    BufferFlushError,

    #[error("Invalid glob pattern: '{0}'")]
    GlobPatternError(String),

    #[error("Failed to build globs from '{0}'")]
    GlobBuildError(String),
}
