//! This module provides the API to work with the Dit version control system

use crate::commit::CommitMgr;
use crate::models::Commit;
use crate::stage::{StageMgr, StagedFiles};
use crate::branch::BranchMgr;
use crate::tree::TreeMgr;
use crate::blob::BlobMgr;
use crate::repo::Repo;
use crate::errors::DitResult;
use std::cell::RefCell;
use std::path::Path;
use std::rc::Rc;

/// Main API for working with the Dit version control system
pub struct Dit {
    blob_mgr: RefCell<BlobMgr>,
    tree_mgr: RefCell<TreeMgr>,
    commit_mgr: RefCell<CommitMgr>,
    stage_mgr: RefCell<StageMgr>,
    branch_mgr: RefCell<BranchMgr>,
}


/// Constructors
impl Dit {
    /// Constructs the object given the project path (inside which the `.dit` is located) \
    /// Constructs all the managers
    pub fn from<P: AsRef<Path>>(project_path: P) -> DitResult<Self> {
        let repo = Rc::new(Repo::init(project_path)?);

        let dit = Self {
            blob_mgr: RefCell::new(BlobMgr::from(repo.clone())),
            tree_mgr: RefCell::new(TreeMgr::from(repo.clone())),
            stage_mgr: RefCell::new(StageMgr::from(repo.clone())?),
            commit_mgr: RefCell::new(CommitMgr::from(repo.clone())),
            branch_mgr: RefCell::new(BranchMgr::from(repo)?),
        };

        Ok(dit)
    }
}


/// Commits
impl Dit {
    /// Creates a commit based on a message and an author
    pub fn commit<S1: Into<String>, S2: Into<String>>(&mut self, author: S1, message: S2)
        -> DitResult<()>
    {
        self.commit_mgr.borrow_mut().create_commit(
            author,
            message,
            &mut self.blob_mgr.borrow_mut(),
            &mut self.tree_mgr.borrow_mut(),
            &mut self.stage_mgr.borrow_mut(),
            &mut self.branch_mgr.borrow_mut(),
        )
    }

    /// Performs a mixed reset to a specific commit. All files not included in
    /// that commit tree stay the same.
    pub fn mixed_reset<S: AsRef<str>>(&mut self, commit: S) -> DitResult<()>
    {
        self.commit_mgr.borrow_mut().mixed_reset(
            commit,
            &mut self.blob_mgr.borrow_mut(),
            &mut self.tree_mgr.borrow_mut(),
            &mut self.branch_mgr.borrow_mut()
        )
    }

    /// Performs a hard reset to a specific commit. The root of the project will be changed
    /// to exactly match the target commit tree (except the ignored files)
    pub fn hard_reset<S: AsRef<str>>(&mut self, commit: S) -> DitResult<()>
    {
        self.commit_mgr.borrow_mut().hard_reset(
            commit,
            &mut self.blob_mgr.borrow_mut(),
            &mut self.tree_mgr.borrow_mut(),
            &mut self.branch_mgr.borrow_mut()
        )
    }

    /// Performs a soft reset to a specific commit. Only changes the head pointer and leaves
    /// the files untouched
    pub fn soft_reset<S: AsRef<str>>(&mut self, commit: S) -> DitResult<()> {
        self.commit_mgr.borrow_mut().soft_reset(commit, &mut self.branch_mgr.borrow_mut())
    }
}


/// Stage
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


/// Branches
impl Dit {
    /// Creates a new branch
    pub fn create_branch<S: AsRef<str>>(&mut self, name: S) -> DitResult<()> {
        self.branch_mgr.borrow_mut().create_branch(name)
    }

    /// Switches to a different branch
    pub fn switch_branch<S: AsRef<str>>(&mut self, name: S, is_hard: bool) -> DitResult<()> {
        self.branch_mgr.borrow_mut().switch_branch(
            name,
            is_hard,
            &mut self.blob_mgr.borrow_mut(),
            &mut self.tree_mgr.borrow_mut(),
            &mut self.commit_mgr.borrow_mut(),
            &mut self.stage_mgr.borrow_mut(),
        )
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
