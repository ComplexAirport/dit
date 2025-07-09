use thiserror::Error;
use dit_core::errors::DitCoreError;


#[derive(Error, Debug)]
pub enum DitCliError {
    #[error("fatal: {0}")]
    DitCoreError(#[from] DitCoreError),

    #[error("Could not get current working directory")]
    CwdError,
}

pub type CliResult<T> = Result<T, DitCliError>;
