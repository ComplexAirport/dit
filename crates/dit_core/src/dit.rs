//! This module provides the API to work with the Dit version control system

use std::cell::RefCell;
use crate::commit::{Commit, CommitMgr};
use crate::stage::{StageMgr, StagedFiles};
use crate::branch::BranchMgr;
use crate::tree::TreeMgr;
use crate::blob::BlobMgr;
use crate::dit_project::DitProject;
use crate::errors::{BranchError, CommitError, DitResult};
use crate::helpers::{create_file_all, get_buf_writer, read_to_string, transfer_data};
use std::path::Path;
use std::rc::Rc;

/// Main API for working with the Dit version control system
pub struct Dit {
    project: Rc<DitProject>,

    blob_mgr: RefCell<BlobMgr>,
    tree_mgr: RefCell<TreeMgr>,
    commit_mgr: RefCell<CommitMgr>,
    stage_mgr: RefCell<StageMgr>,
    branch_mgr: RefCell<BranchMgr>,
}

/// Constructors
impl Dit {
    /// Constructs the object given the project path (inside which the `.dit` is located) \
    /// Creates commit, tree and blob managers
    pub fn from<P: AsRef<Path>>(project_path: P) -> DitResult<Self> {
        let project = Rc::new(DitProject::init(project_path)?);

        let dit = Self {
            project: project.clone(),
            blob_mgr: RefCell::new(BlobMgr::from(project.clone())),
            tree_mgr: RefCell::new(TreeMgr::from(project.clone())),
            stage_mgr: RefCell::new(StageMgr::from(project.clone())?),
            commit_mgr: RefCell::new(CommitMgr::from(project.clone())),
            branch_mgr: RefCell::new(BranchMgr::from(project.clone())?),
        };

        Ok(dit)
    }
}

/// Commiting
impl Dit {
    /// Commits the changes given the commit author and the message
    pub fn commit<S1, S2>(&mut self, author: S1, message: S2) -> DitResult<()>
    where
        S1: Into<String>,
        S2: Into<String>,
    {
        let mut blob_mgr = self.blob_mgr.borrow_mut();
        let mut tree_mgr = self.tree_mgr.borrow_mut();
        let commit_mgr = self.commit_mgr.borrow_mut();

        let stage_mgr = self.stage_mgr.borrow();
        let staged_files = stage_mgr.staged_files();

        let parent = self.branch_mgr.borrow().get_head_commit().cloned();

        let author = author.into();
        let message = message.into();

        let commit_hash = commit_mgr.create_commit(
            author, message, staged_files, parent, &mut blob_mgr, &mut tree_mgr
        )?;

        self.branch_mgr.borrow_mut().set_head_commit(commit_hash)?;

        // Clean up the stage
        self.stage_mgr.borrow_mut().clear_stage()?;

        Ok(())
    }

    /// Performs a mixed reset to a specific commit. All files not included in
    /// that commit tree stay the same.
    pub fn mixed_reset<S: AsRef<str>>(&mut self, commit: S) -> DitResult<()> {
        let commit = commit.as_ref();
        let head = self.branch_mgr.borrow().get_head_commit().cloned();

        // Check if the commit is reachable
        if let Some(head) = head {
            if !self.commit_mgr.borrow().is_parent(commit, &head)? {
                return Err(
                    CommitError::UnreachableCommitError(commit.to_string(), head.to_string()).into()
                );
            }
        }

        // Now we know that the commit is reachable.
        let commit = self.commit_mgr.borrow().get_commit(commit)?;
        let tree = self.tree_mgr.borrow().get_tree(commit.tree)?;
        let files = tree.files;

        for (rel_path, blob_hash) in files {
            let mut reader = self.blob_mgr.borrow().get_blob_reader(blob_hash)?;

            let abs_path = self.project.get_absolute_path(&rel_path)?;
            create_file_all(&abs_path)?;
            let mut writer = get_buf_writer(&abs_path)?;

            transfer_data(&mut reader, &mut writer, &abs_path)?;
        }

        self.branch_mgr.borrow_mut().set_head_commit(commit.hash)?;

        Ok(())
    }
}


/// Staging
impl Dit {
    /// Stages a file given the file path
    pub fn stage<P: AsRef<Path>>(&mut self, path: P) -> DitResult<()> {
        self.stage_mgr.borrow_mut().stage_file(path)
    }

    /// Unstages the file given the file path
    pub fn unstage<P: AsRef<Path>>(&mut self, path: P) -> DitResult<()> {
        self.stage_mgr.borrow_mut().unstage_file(path)
    }

    /// Clears the stage
    pub fn clear_stage(&mut self) -> DitResult<()> {
        self.stage_mgr.borrow_mut().clear_stage()
    }
}


/// Branching
impl Dit {
    /// Creates a new branch
    pub fn create_branch<S: AsRef<str>>(&mut self, name: S) -> DitResult<()> {
        self.branch_mgr.borrow_mut().create_branch(name)
    }

    /// Switches to a different branch
    pub fn switch_branch<S: AsRef<str>>(&mut self, name: S, is_hard: bool) -> DitResult<()> {
        let name = name.as_ref();
        let (exists, path) = self.branch_mgr.borrow().find_branch(name);

        if !exists {
            return Err(BranchError::BranchDoesNotExist(name.to_string()).into());
        }

        if self.stage_mgr.borrow().is_staged() {
            if !is_hard {
                return Err(BranchError::CannotSwitchBranches(name.to_string()).into());
            } else {
                // if the hard mode is set, any staged changes will be cleared
                self.stage_mgr.borrow_mut().clear_stage()?;
            }
        }

        // Get the commit tree
        let target_commit_hash = read_to_string(path)?;
        let target_commit = self.commit_mgr.borrow().get_commit(&target_commit_hash)?;
        let files = self.tree_mgr.borrow().get_tree(target_commit.tree)?.files;

        // Remove the current project root
        // self.clear_root()?;

        // self.branch_mgr.set_head_commit(target_commit_hash)?;

        Ok(())
    }
}


/// Getters
impl Dit {
    /// Returns the name of the current branch
    pub fn branch(&self) -> Option<String> {
        self.branch_mgr.borrow().get_current_branch().cloned()
    }

    /// Returns the hash of the current HEAD commit
    pub fn head_commit(&self) -> Option<String> {
        self.branch_mgr.borrow().get_head_commit().cloned()
    }

    /// Returns the commit history
    pub fn history(&mut self, mut count: isize) -> DitResult<Vec<Commit>> {
        if count < 0 {
            count = isize::MAX;
        }

        let mut commits = Vec::new();
        let mut head_commit = self.branch_mgr.borrow().get_head_commit().cloned();

        while let Some(head) = &head_commit {
            if count == 0 {
                break;
            }

            let commit = self.commit_mgr.borrow().get_commit(head)?;
            head_commit = commit.parent.clone();
            commits.push(commit);

            count -= 1;
        }

        Ok(commits)
    }

    /// Returns staged files
    pub fn with_staged_files(&self, f: impl FnOnce(&StagedFiles)) {
        f(self.stage_mgr.borrow().staged_files());
    }
}

