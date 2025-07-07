use std::io;
use std::path::PathBuf;
use crate::commits::CommitMgr;
use crate::constants::{DIT_ROOT};
use crate::trees::{StagedFiles};

/// Main API for working with the Dit version control system
pub struct Dit {
    /// Represents the project directory, where the `.dit` is located
    project_path: PathBuf,

    /// Represents the `.dit` path, [`DIT_ROOT`]
    root: PathBuf,

    /// Represents the commit manager
    commit_mgr: CommitMgr,

    /// Tree builder for staging changes
    staged_files: StagedFiles,
}


/// Constructors
impl Dit {
    /// Constructs the object given the project path (inside which the `.dit` is located) \
    /// Creates commit, tree and blob managers
    pub fn from_project<P: Into<PathBuf>>(project_path: P) -> io::Result<Self> {
        let project_path = project_path.into();
        let root= project_path.join(DIT_ROOT);

        if !root.exists() {
            std::fs::create_dir_all(&root)?;
        }

        let commit_mgr = CommitMgr::from_project(&project_path)?;
        let staged_files = StagedFiles::new();

        Ok(Self{
            project_path,
            root,
            commit_mgr,
            staged_files
        })
    }
}


/// Manage commits
impl Dit {
    pub fn commit<S1, S2>(
        &mut self,
        author: S1,
        message: S2,
    ) -> io::Result<()>
    where S1: Into<String>, S2: Into<String>
    {
        let author = author.into();
        let message = message.into();
        let parent_commit_hash = self.commit_mgr.read_head()?;
        self.commit_mgr.create_commit(author, message, self.staged_files.clone(), parent_commit_hash)?;
        Ok(())
    }
}


/// Manage file staging/unstaging
impl Dit {
    pub fn stage_file<P: Into<PathBuf>>(&mut self, file_path: P) -> io::Result<()> {
        // add the file to staged files
        self.staged_files.add_file(file_path)?;

        // register the changes
        self.commit_mgr.register_staged_files(self.staged_files.clone())?;

        Ok(())
    }

    pub fn unstage_file<P: Into<PathBuf>>(&mut self, file_path: P) -> io::Result<()> {
        // remove the file from the staged files
        self.staged_files.remove_file(file_path);

        // register the changes
        self.commit_mgr.register_staged_files(self.staged_files.clone())?;

        Ok(())
    }
}