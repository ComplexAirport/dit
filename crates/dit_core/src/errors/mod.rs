use thiserror::Error;

mod blob;
mod tree;
mod commit;
mod stage;
mod branch;
mod project;
mod fs;
mod other;

pub use self::{
    blob::BlobError,
    tree::TreeError,
    commit::CommitError,
    stage::StagingError,
    branch::BranchError,
    project::ProjectError,
    fs::FsError,
    other::OtherError,
};

#[derive(Error, Debug)]
pub enum DitCoreError {
    #[error("branch error: {0}")]
    BranchError(#[from] BranchError),

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
