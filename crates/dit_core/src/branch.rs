use crate::dit_project::DitProject;
use crate::errors::{BranchError, DitResult};
use crate::helpers::{read_to_string, write_to_file};
use crate::constants::DEFAULT_BRANCH;
use std::rc::Rc;

pub struct BranchMgr {
    project: Rc<DitProject>,

    /// Represents the current branch name
    curr_branch: Option<String>,

    /// Represents the current commit head (the hash of the current commit)
    curr_commit: Option<String>,
}

/// Constructors
impl BranchMgr {
    pub fn from(project: Rc<DitProject>) -> DitResult<Self> {
        let mut branch_mgr = Self {
            project,
            curr_branch: None,
            curr_commit: None,
        };

        Self::load(&mut branch_mgr)?;

        Ok(branch_mgr)
    }
}

/// API
impl BranchMgr {
    /// Creates a new branch based on the given name
    ///
    /// Returns an error if a branch with a such name already exists
    pub fn create_branch<S: AsRef<str>>(&mut self, name: S) -> DitResult<()> {
        let name = name.as_ref();
        let path = self.project.branches().join(name);

        if path.exists() {
            return Err(BranchError::BranchAlreadyExists(name.to_string()).into())
        }

        match &self.curr_commit {
            None => {
                write_to_file(&path, "")?;
            }

            Some(curr_commit) => {
                write_to_file(&path, curr_commit)?;
            }
        }

        self.curr_branch = Some(name.to_string());
        self.store()?;

        Ok(())
    }

    /// Returns the name of the current branch
    pub fn get_current_branch(&self) -> Option<&String> {
        self.curr_branch.as_ref()
    }

    /// Sets the head commit to a new value
    pub fn set_head_commit<S: AsRef<str>>(&mut self, commit: S) -> DitResult<()> {
        let commit = commit.as_ref();
        self.curr_commit = Some(commit.to_string());
        self.store()?;
        Ok(())
    }

    /// Returns the hash of the current commit
    pub fn get_head_commit(&self) -> Option<&String> { self.curr_commit.as_ref() }
}


/// Private helper methods
impl BranchMgr {
    /// Loads the current branch and(or) commit based on [`HEAD_FILE`]
    ///
    /// [`HEAD_FILE`]: crate::constants::HEAD_FILE
    fn load(&mut self) -> DitResult<()> {
        let path = self.project.head_file();
        let head = read_to_string(path)?;

        // if the head starts with ":", then it references a commit and not a branch
        if let Some(head) = head.strip_prefix(':') {
            self.curr_commit = Some(head.to_string());
            self.curr_branch = None;
        } else if head.is_empty() {
            self.curr_branch = None;
            self.curr_commit = None;
        } else {
            let path = self.project.branches().join(&head);
            let commit = read_to_string(&path)?;
            if commit.is_empty() {
                self.curr_commit = None;
            } else {
                self.curr_commit = Some(commit);
            }
            self.curr_branch = Some(head);
        }

        Ok(())
    }

    /// Updates the [`HEAD_FILE`] based on the current branch and(or) commit stored in self
    ///
    /// [`HEAD_FILE`]: crate::constants::HEAD_FILE
    fn store(&mut self) -> DitResult<()> {
        let head_file = self.project.head_file();

        if let Some(curr_branch) = &self.curr_branch {
            write_to_file(head_file, curr_branch)?;
            let branch_file = self.project.branches().join(curr_branch);
            match &self.curr_commit {
                Some(curr_commit) => write_to_file(&branch_file, curr_commit)?,
                None => write_to_file(&branch_file, "")?,
            }
        } else {
            match &self.curr_commit {
                Some(head) => write_to_file(head_file, format!(":{}", head))?,
                None => write_to_file(head_file, "")?,
            }
        }

        Ok(())
    }
}

