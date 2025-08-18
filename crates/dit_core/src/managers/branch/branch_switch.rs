use crate::managers::blob::BlobMgr;
use crate::managers::tree::TreeMgr;
use crate::managers::commit::CommitMgr;
use crate::managers::index::IndexMgr;
use crate::managers::branch::BranchMgr;
use crate::managers::ignore::IgnoreMgr;
use crate::errors::{BranchError, DitResult};
use crate::helpers::{create_file_all, read_to_string};
use std::fs;
use std::collections::BTreeMap;
use rayon::prelude::*;

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
                fs::write(&path, "")?;
            }

            Some(curr_commit) => {
                fs::write(&path, curr_commit)?;
            }
        }

        self.set_current_branch(name)
    }


    /// Switches to a different branch
    pub fn switch_branch<S: AsRef<str>>(
        &mut self,
        name: S,
        is_hard: bool,
        blob_mgr: &BlobMgr,
        tree_mgr: &TreeMgr,
        commit_mgr: &CommitMgr,
        index_mgr: &mut IndexMgr,
        ignore_mgr: &IgnoreMgr,
    ) -> DitResult<()> {
        let name = name.as_ref();
        let (exists, path) = self.find_branch(name);

        if !exists {
            return Err(BranchError::BranchDoesNotExist(name.to_string()).into());
        }

        if is_hard {
            self.prepare_stage_for_switch_hard(tree_mgr, commit_mgr, index_mgr, self)?;
        } else { // todo: change this behavior?
            self.prepare_stage_for_switch_soft(name, tree_mgr, commit_mgr, index_mgr, self)?;
        }

        // Get the commit tree
        let target_commit_hash = read_to_string(&path)?;
        let files = if target_commit_hash.is_empty() {
            BTreeMap::new()
        } else {
            let target_commit = commit_mgr.get_commit(&target_commit_hash)?;
            tree_mgr.get_tree(target_commit.tree)?.index.files
        };

        // Remove the current project root
        ignore_mgr.clear_dir(self.repo.repo_path())?;

        // Create the files in the commit
        files.into_par_iter()
            .try_for_each(|(rel_path, entry)| -> DitResult<()> {
                create_file_all(&rel_path)?;
                let src = blob_mgr.get_blob_path(entry.hash);
                fs::copy(&src, &rel_path)?;
                Ok(())
            })?;

        // Set heads to the branch
        self.set_head(name, target_commit_hash)
    }
}


/// Private
impl BranchMgr {
    /// Resets the index if it's not empty
    pub(super) fn prepare_stage_for_switch_hard(
        &self,
        tree_mgr: &TreeMgr,
        commit_mgr: &CommitMgr,
        index_mgr: &mut IndexMgr,
        branch_mgr: &BranchMgr,
    ) -> DitResult<()>
    {
        index_mgr.unstage_all(tree_mgr, commit_mgr, branch_mgr)
    }

    /// If the index is not empty, returns an error. Otherwise, everything is OK.
    pub(super) fn prepare_stage_for_switch_soft<S>(
        &self,
        switch_to_branch: S,
        tree_mgr: &TreeMgr,
        commit_mgr: &CommitMgr,
        index_mgr: &mut IndexMgr,
        branch_mgr: &BranchMgr,
    ) -> DitResult<()>
    where S: Into<String>
    {
        if index_mgr.are_tracked_changes(tree_mgr, commit_mgr, branch_mgr)? {
            Err(BranchError::CannotSwitchBranches(switch_to_branch.into()).into())
        } else {
            Ok(())
        }
    }
}
