use thiserror::Error;

/// Other errors
#[derive(Error, Debug)]
pub enum OtherError {
    #[error("Current system time is earlier then the unix epoch time.")]
    TimeWentBackwardsError,
}
