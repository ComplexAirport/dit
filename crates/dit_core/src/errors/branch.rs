use thiserror::Error;

/// Errors related to branches
#[derive(Error, Debug)]
pub enum BranchError {
    #[error("A branch with name '{0}' already exists")]
    BranchAlreadyExists(String),

    #[error("A branch with name '{0}' doesn't exists")]
    BranchDoesNotExist(String),

    #[error("Invalid branch name '{0}'")]
    InvalidBranchName(String),

    #[error("Cannot switch to branch '{0}' because there are staged changes.
    Commit the changes, stash them or use hard switching.")]
    CannotSwitchBranches(String),

    #[error("Cannot merge to branch '{0}' because the head is in a detached head state.")]
    CannotMergeToDetachedHead(String),

    #[error("Merging branches which are not ancestors is not supported yet")]
    MergeNotSupported, // todo
}
