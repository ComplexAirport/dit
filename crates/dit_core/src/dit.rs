//! This module provides the API to work with the Dit version control system
use crate::commit::{Commit, CommitMgr};
use crate::stage::{StageMgr, StagedFiles};
use crate::branch::BranchMgr;
use crate::tree::TreeMgr;
use crate::blob::BlobMgr;
use crate::dit_project::DitProject;
use crate::errors::{CommitError, DitResult};
use crate::helpers::{create_file_all, get_buf_writer, transfer_data};
use std::path::Path;
use std::rc::Rc;

/// Main API for working with the Dit version control system
pub struct Dit {
    project: Rc<DitProject>,

    /// Represents the commit manager
    commit_mgr: CommitMgr,

    /// Represents the stage manager
    stage_mgr: StageMgr,

    /// Represents the branches manager
    branch_mgr: BranchMgr,
}

/// Constructors
impl Dit {
    /// Constructs the object given the project path (inside which the `.dit` is located) \
    /// Creates commit, tree and blob managers
    pub fn from<P: AsRef<Path>>(project_path: P) -> DitResult<Self> {
        let project = Rc::new(DitProject::init(project_path)?);

        let commit_mgr = CommitMgr::from(project.clone());
        let stage_mgr = StageMgr::from(project.clone())?;
        let branch_mgr = BranchMgr::from(project.clone())?;

        let dit = Self {
            project,
            commit_mgr,
            stage_mgr,
            branch_mgr,
        };

        Ok(dit)
    }
}

/// Dit API
impl Dit {
    /// Commits the changes given the commit author and the message
    pub fn commit<S1, S2>(&mut self, author: S1, message: S2) -> DitResult<()>
    where
        S1: Into<String>,
        S2: Into<String>,
    {
        let author = author.into();
        let message = message.into();
        let staged_files = self.stage_mgr.staged_files();
        let parent = self.branch_mgr.get_head_commit().cloned();

        let commit_hash = self.commit_mgr.create_commit(
            author, message, staged_files, parent
        )?;

        self.branch_mgr.set_head_commit(commit_hash)?;

        // Clean up the stage
        self.stage_mgr.clear_stage()?;

        Ok(())
    }

    /// Stages a file given the file path
    pub fn stage<P: AsRef<Path>>(&mut self, path: P) -> DitResult<()> {
        self.stage_mgr.stage_file(path)
    }

    /// Unstages the file given the file path
    pub fn unstage<P: AsRef<Path>>(&mut self, path: P) -> DitResult<()> {
        self.stage_mgr.unstage_file(path)
    }

    /// Creates a new branch
    pub fn create_branch<S: AsRef<str>>(&mut self, name: S) -> DitResult<()> {
        self.branch_mgr.create_branch(name)
    }

    /// Performs a mixed reset to a specific commit. All files not included in 
    /// that commit tree stay the same.
    pub fn mixed_reset<S: AsRef<str>>(&mut self, commit: S) -> DitResult<()> {
        let commit = commit.as_ref();
        let head = self.branch_mgr.get_head_commit().cloned();

        // Check if the commit is reachable
        if let Some(head) = head {
            if !self.commit_mgr.is_parent(commit, &head)? {
                return Err(
                    CommitError::UnreachableCommitError(commit.to_string(), head.to_string()).into()
                );
            }
        }

        // Now we know that the commit is reachable.
        let commit = self.commit_mgr.get_commit(commit)?;
        let tree = self.tree_mgr().get_tree(commit.tree)?;
        let files = tree.files;

        for (rel_path, blob_hash) in files {
            let mut reader = self.blob_mgr().get_blob_reader(blob_hash)?;

            let abs_path = self.project.get_absolute_path(&rel_path)?;
            create_file_all(&abs_path)?;
            let mut writer = get_buf_writer(&abs_path)?;

            transfer_data(&mut reader, &mut writer, &abs_path)?;
        }

        self.branch_mgr.set_head_commit(commit.hash)?;

        Ok(())
    }
}

/// Getters
impl Dit {
    /// Returns the name of the current branch
    pub fn branch(&self) -> Option<String> {
        self.branch_mgr.get_current_branch().cloned()
    }

    /// Returns the hash of the current HEAD commit
    pub fn head_commit(&self) -> Option<String> { self.branch_mgr.get_head_commit().cloned() }

    /// Returns the commit history
    pub fn history(&mut self, mut count: isize) -> DitResult<Vec<Commit>> {
        if count < 0 {
            count = isize::MAX;
        }

        let mut commits = Vec::new();
        let mut head_commit = self.branch_mgr.get_head_commit().cloned();

        while let Some(head) = &head_commit {
            if count == 0 {
                break;
            }

            let commit = self.commit_mgr.get_commit(head)?;
            head_commit = commit.parent.clone();
            commits.push(commit);

            count -= 1;
        }

        Ok(commits)
    }

    /// Returns staged files
    pub fn staged_files(&mut self) -> DitResult<&StagedFiles> {
        let files = self.stage_mgr.staged_files();
        Ok(files)
    }
}

/// Private helpers
impl Dit {
    /// Returns the project's [`TreeMgr`]
    pub const fn tree_mgr(&mut self) -> &mut TreeMgr {
        &mut self.commit_mgr.tree_mgr
    }

    /// Returns the project's [`BlobMgr`]
    pub const fn blob_mgr(&mut self) -> &mut BlobMgr {
        &mut self.tree_mgr().blob_mgr
    }
}
