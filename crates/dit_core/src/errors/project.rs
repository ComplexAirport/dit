use thiserror::Error;

/// General dit project-related errors
#[derive(Error, Debug)]
pub enum ProjectError {
    #[error("The file '{0}' is not inside the project")]
    FileNotInProject(String),

    #[error("The given project path '{0}' is not a directory")]
    ProjectPathNotADirectory(String),

    #[error("Failed to create .dit project subdirectory '{0}'")]
    SubDirCreationError(String),

    #[error("Failed to create .dit project file '{0}'")]
    SubFileCreationError(String),
}
