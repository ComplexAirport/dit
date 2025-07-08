//! This module provides the API to work with the Dit version control system

use crate::commit::{Commit, CommitMgr};
use crate::dit_project::DitProject;
use crate::stage::{StageMgr, StagedFiles};
use std::path::Path;
use std::rc::Rc;
use std::{fs, io};

/// Main API for working with the Dit version control system
pub struct Dit {
    project: Rc<DitProject>,

    /// Represents the commit manager
    commit_mgr: CommitMgr,

    /// Represents the stage manager
    stage_mgr: StageMgr,

    /// Head hash
    head: Option<String>,
}

/// Constructors
impl Dit {
    /// Constructs the object given the project path (inside which the `.dit` is located) \
    /// Creates commit, tree and blob managers
    pub fn from<P: AsRef<Path>>(project_path: P) -> io::Result<Self> {
        let project = Rc::new(DitProject::init(project_path)?);
        let commit_mgr = CommitMgr::from(project.clone())?;
        let stage_mgr = StageMgr::from(project.clone())?;

        let mut dit = Self {
            project,
            commit_mgr,
            stage_mgr,
            head: None,
        };

        Self::load_head(&mut dit)?;

        Ok(dit)
    }
}

/// Dit API
impl Dit {
    /// Commits the changes given the commit author and the message
    pub fn commit<S1, S2>(&mut self, author: S1, message: S2) -> io::Result<()>
    where
        S1: Into<String>,
        S2: Into<String>,
    {
        self.load_head()?; // load the head to access parent hash

        let author = author.into();
        let message = message.into();
        let staged_files = self.stage_mgr.staged_files();

        let commit_hash = self.commit_mgr.create_commit(
            author, message, staged_files, self.head.clone()
        )?;

        self.head = Some(commit_hash);
        self.update_head()?;

        // Clean up the stage
        self.stage_mgr.clear_stage()?;

        Ok(())
    }

    /// Stages a file given the file path
    pub fn stage<P: AsRef<Path>>(&mut self, path: P) -> io::Result<()> {
        self.stage_mgr.stage_file(path)
    }

    /// Unstages the file given the file path
    pub fn unstage<P: AsRef<Path>>(&mut self, path: P) -> io::Result<()> {
        self.stage_mgr.unstage_file(path)
    }

    /// Returns the commit history
    pub fn history(&mut self, mut count: usize) -> io::Result<Vec<Commit>> {
        self.update_head()?;
        let mut commits = Vec::new();

        let mut head_hash = self.head.clone();

        while let Some(head) = &head_hash {
            if count == 0 {
                break;
            }

            let commit = self.commit_mgr.get_commit(head)?;
            head_hash = commit.parent.clone();
            commits.push(commit);

            count -= 1;
        }

        Ok(commits)
    }

    /// Returns staged files
    pub fn staged_files(&mut self) -> io::Result<&StagedFiles> {
        let files = self.stage_mgr.staged_files();
        Ok(files)
    }
}

/// Manage the head ([`HEAD_FILE`])
///
/// [`HEAD_FILE`]: crate::constants::HEAD_FILE
impl Dit {
    /// Loads the head stored in self based on [`HEAD_FILE`]
    ///
    /// [`HEAD_FILE`]: crate::constants::HEAD_FILE
    fn load_head(&mut self) -> io::Result<()> {
        let path = self.project.head_file();
        let head = fs::read_to_string(path)?;

        if head.is_empty() {
            self.head = None;
        } else {
            self.head = Some(head);
        }

        Ok(())
    }

    /// Updates the [`HEAD_FILE`] based on head stored in self
    ///
    /// [`HEAD_FILE`]: crate::constants::HEAD_FILE
    fn update_head(&mut self) -> io::Result<()> {
        let path = self.project.head_file();

        match &self.head {
            Some(head) => fs::write(path, head)?,
            None => fs::write(path, "")?,
        }

        Ok(())
    }
}
