use crate::managers::blob::BlobMgr;
use crate::managers::tree::TreeMgr;
use crate::managers::commit::CommitMgr;
use crate::managers::stage::StageMgr;
use crate::managers::branch::BranchMgr;
use crate::errors::{BranchError, DitResult};
use crate::helpers::{clear_dir_except, create_file_all, get_buf_writer, read_to_string, transfer_data, write_to_file};
use std::collections::BTreeMap;


/// Public
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
}


/// Private
impl BranchMgr {
    /// Resets the stage if it's not empty
    pub(super) fn prepare_stage_for_switch_hard(&self, stage_mgr: &mut StageMgr) -> DitResult<()>
    {
        if stage_mgr.is_staged() {
            stage_mgr.clear_stage()?;
        }
        Ok(())
    }

    /// If the stage is not empty, returns an error. Otherwise, everything is OK.
    pub(super) fn prepare_stage_for_switch_soft<S>(
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
