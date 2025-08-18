use thiserror::Error;

mod blob;
mod tree;
mod commit;
mod index;
mod branch;
mod project;
mod fs;
mod other;
mod config;

pub use self::{
    blob::BlobError,
    tree::TreeError,
    commit::CommitError,
    index::IndexError,
    branch::BranchError,
    project::ProjectError,
    fs::FsError,
    other::OtherError,
    config::ConfigError,
};

#[derive(Error, Debug)]
pub enum DitCoreError {
    #[error("branch error: {0}")]
    BranchError(#[from] BranchError),

    #[error("index error: {0}")]
    IndexError(#[from] IndexError),

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

    #[error("io error: {0}")]
    IOError(#[from] std::io::Error),

    #[error("config error: {0}")]
    ConfigError(#[from] ConfigError),

    #[error("serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),

    #[error("error: {0}")]
    OtherError(#[from] OtherError),
}

pub type DitResult<T> = Result<T, DitCoreError>;
