use std::collections::BTreeMap;
use crate::repo::Repo;
use crate::errors::{BranchError, DitResult};
use crate::helpers::{clear_dir_except, create_file_all, get_buf_writer, read_to_string, transfer_data, write_to_file};
use crate::blob::BlobMgr;
use crate::tree::TreeMgr;
use crate::commit::CommitMgr;
use crate::stage::StageMgr;
use std::path::PathBuf;
use std::rc::Rc;

pub struct BranchMgr {
    repo: Rc<Repo>,

    /// Represents the current branch name
    curr_branch: Option<String>,

    /// Represents the current commit head (the hash of the current commit)
    curr_commit: Option<String>,
}

/// Constructors
impl BranchMgr {
    pub fn from(repo: Rc<Repo>) -> DitResult<Self> {
        let mut branch_mgr = Self {
            repo,
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
    pub fn create_branch<S: AsRef<str>>(
        &mut self, name: S)
        -> DitResult<()> {
        let name = name.as_ref();

        if !name.chars().all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_') {
            return Err(BranchError::InvalidBranchName(name.to_string()).into())
        }

        let (exists, path) = self.find_branch(name);
        if exists {
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

        self.set_current_branch(name)?;

        Ok(())
    }

    /// Switches to a different branch
    pub fn switch_branch<S: AsRef<str>>(
        &mut self,
        name: S,
        is_hard: bool,
        blob_mgr: &mut BlobMgr,
        tree_mgr: &mut TreeMgr,
        commit_mgr: &mut CommitMgr,
        stage_mgr: &mut StageMgr,
    ) -> DitResult<()> {
        let name = name.as_ref();
        let (exists, path) = self.find_branch(name);

        if !exists {
            return Err(BranchError::BranchDoesNotExist(name.to_string()).into());
        }

        if is_hard {
            self.prepare_stage_for_switch_hard(stage_mgr)?;
        } else { // todo: change this behavior?
            self.prepare_stage_for_switch_soft(name, stage_mgr)?;
        }

        // Get the commit tree
        let target_commit_hash = read_to_string(path)?;
        let files = if target_commit_hash.is_empty() {
            BTreeMap::new()
        } else {
            let target_commit = commit_mgr.get_commit(&target_commit_hash)?;
            tree_mgr.get_tree(target_commit.tree)?.files
        };

        // Remove the current project root
        clear_dir_except(self.repo.repo_path(), [".dit"])?; // todo

        for (rel_path, blob_hash) in files {
            create_file_all(&rel_path)?;
            let mut blob_reader = blob_mgr.get_blob_reader(blob_hash)?;
            let mut writer = get_buf_writer(&rel_path)?;
            transfer_data(&mut blob_reader, &mut writer, rel_path)?;
        }

        self.set_head_commit(target_commit_hash)?;

        // todo: this definitely needs improvement
        Ok(())
    }

    /// Sets the head branch to a new value
    pub fn set_current_branch<S: AsRef<str>>(&mut self, branch: S) -> DitResult<()> {
        let branch = branch.as_ref();
        self.curr_branch = Some(branch.to_string());
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

    /// Returns a bool indicating whether the branch exists or not and
    /// the path to that branch file
    pub fn find_branch<S: AsRef<str>>(&self, name: S) -> (bool, PathBuf) {
        let name = name.as_ref();
        let path = self.repo.branches().join(name);

        (path.exists(), path)
    }
}


/// Private helper methods
impl BranchMgr {
    /// Loads the current branch and(or) commit based on [`HEAD_FILE`]
    ///
    /// [`HEAD_FILE`]: crate::project_structure::HEAD_FILE
    fn load(&mut self) -> DitResult<()> {
        let path = self.repo.head_file();
        let head = read_to_string(path)?;

        // if the head starts with ":", then it references a commit and not a branch
        if let Some(head) = head.strip_prefix(':') {
            self.curr_commit = Some(head.to_string());
            self.curr_branch = None;
        } else if head.is_empty() {
            self.curr_branch = None;
            self.curr_commit = None;
        } else {
            let path = self.repo.branches().join(&head);
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
    /// [`HEAD_FILE`]: crate::project_structure::HEAD_FILE
    fn store(&mut self) -> DitResult<()> {
        let head_file = self.repo.head_file();

        if let Some(curr_branch) = &self.curr_branch {
            write_to_file(head_file, curr_branch)?;
            let branch_file = self.repo.branches().join(curr_branch);
            match &self.curr_commit {
                Some(curr_commit) => write_to_file(&branch_file, curr_commit)?,
                None => write_to_file(&branch_file, "")?,
            }
        } else {
            match &self.curr_commit {
                Some(head) => write_to_file(head_file, format!(":{head}"))?,
                None => write_to_file(head_file, "")?,
            }
        }

        Ok(())
    }

    /// Resets the stage if it's not empty
    fn prepare_stage_for_switch_hard(&self, stage_mgr: &mut StageMgr) -> DitResult<()>
    {
        if stage_mgr.is_staged() {
            stage_mgr.clear_stage()?;
        }
        Ok(())
    }

    /// If the stage is not empty, returns an error. Otherwise, everything is OK.
    fn prepare_stage_for_switch_soft<S>(
        &self,
        switch_to_branch: S,
        stage_mgr: &mut StageMgr
    ) -> DitResult<()>
    where S: Into<String>
    {
        if stage_mgr.is_staged() {
            Err(BranchError::CannotSwitchBranches(switch_to_branch.into()).into())
        } else {
            Ok(())
        }
    }
}
