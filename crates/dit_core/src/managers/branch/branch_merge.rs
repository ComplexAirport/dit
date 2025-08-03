use crate::managers::blob::BlobMgr;
use crate::managers::branch::BranchMgr;
use crate::managers::commit::CommitMgr;
use crate::managers::tree::TreeMgr;
use crate::errors::{BranchError, DitResult};
use std::collections::BTreeMap;
use std::path::PathBuf;

/// Public
impl BranchMgr {
    /// Merges two branches
    pub fn merge_to<S: AsRef<str>>(&mut self, merge_to: S) -> DitResult<()> {
        let merge_to = merge_to.as_ref();
        let merge_from = self.curr_branch.as_ref()
            .ok_or_else(|| BranchError::CannotMergeToDetachedHead(merge_to.to_string()))?;

        Ok(())
    }
}


/// Private
impl BranchMgr {
    /// Tries to merge to branches
    pub(super) fn merge_branches<S1, S2>(
        &mut self,
        merge_from: S1,
        merge_to: S2,
        blob_mgr: &BlobMgr,
        tree_mgr: &TreeMgr,
        commit_mgr: &CommitMgr,
    ) -> DitResult<()>
    where S1: Into<String>, S2: Into<String> {
        // todo: first if (from, to) are ancestors or vice-versa
        let merge_from = merge_from.into();
        let merge_to = merge_to.into();

        let common_ancestor = commit_mgr.common_ancestor(&merge_from, &merge_to)?;
        if common_ancestor.is_none() {
            // todo: make it possible to merge branches with no common ancestors
            return Err(BranchError::CannotMergeBranchesWithNoCommonAncestors(merge_from, merge_to).into());
        }
        let common_ancestor = common_ancestor.unwrap();
        let common_commit = commit_mgr.get_commit(&common_ancestor)?;
        let common_tree = tree_mgr.get_tree(common_commit.tree)?;

        let merge_from_branch = self.get_head_commit_of_branch(&merge_from)?.unwrap();
        let merge_from_commit = commit_mgr.get_commit(&merge_from_branch)?;
        let merge_from_tree = tree_mgr.get_tree(merge_from_commit.tree)?;

        let merge_to_branch = self.get_head_commit_of_branch(&merge_to)?.unwrap();
        let merge_to_commit = commit_mgr.get_commit(&merge_to_branch)?;
        let merge_to_tree = tree_mgr.get_tree(merge_to_commit.tree)?;

        // this map represents the following data structure:
        // relative path: (
        //      Option<blob_hash_from_common_ancestor>,
        //      Option<blob_hash_from_merge_from_tree>,
        //      Option<blob_hash_from_merge_to_tree>,
        // )
        let mut changes: BTreeMap<PathBuf, (Option<String>, Option<String>, Option<String>)>
            = BTreeMap::new();

        for (rel_path, blob_hash) in common_tree.files {
            changes.insert(rel_path, (Some(blob_hash), None, None));
        }

        for (rel_path, blob_hash) in merge_from_tree.files {
            if let Some(change) = changes.get_mut(&rel_path) {
                change.1 = Some(blob_hash);
            } else {
                changes.insert(rel_path, (None, Some(blob_hash), None));
            }
        }

        for (rel_path, blob_hash) in merge_to_tree.files {
            if let Some(change) = changes.get_mut(&rel_path) {
                change.2 = Some(blob_hash);
            } else {
                changes.insert(rel_path, (None, None, Some(blob_hash)));
            }
        }

        Ok(())
    }
}
