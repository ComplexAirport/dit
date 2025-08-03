use crate::managers::blob::BlobMgr;
use crate::managers::branch::BranchMgr;
use crate::managers::commit::CommitMgr;
use crate::managers::tree::TreeMgr;
use crate::errors::{BranchError, DitResult};
use std::collections::BTreeMap;
use std::path::PathBuf;
use crate::models::Tree;

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
        from: S1,
        to: S2,
        blob_mgr: &BlobMgr,
        tree_mgr: &TreeMgr,
        commit_mgr: &CommitMgr,
    ) -> DitResult<()>
    where S1: Into<String>, S2: Into<String> {
        let from = from.into();
        let to = to.into();

        // A -> B -> C -> D -> E -> F
        //      ^ BRANCH1           ^ BRANCH2

        // Case 1: Merge BRANCH1 into BRANCH2
        // do nothing, BRANCH2 is already up to date
        if commit_mgr.is_ancestor(&from, &to)? {
            return Ok(());
        }
        // Case 2: Merge BRANCH2 into BRANCH1
        // In this case, simply move the BRANCH1 pointer to point to BRANCH2 head
        // A -> B -> C -> D -> E -> F
        //                          ^ BRANCH1, BRANCH2
        else if commit_mgr.is_ancestor(&to, &from)? {
            let merge_from_commit = self.get_branch_head(&from)?
                .unwrap_or_else(String::new);
            self.set_branch_head(&to, merge_from_commit)?;
            return Ok(());
        }

        // Otherwise, we should find the common ancestor and merge based on it
        let common_ancestor = commit_mgr.common_ancestor(&from, &to)?;
        if common_ancestor.is_none() {
            // todo: make it possible to merge branches with no common ancestors
            return Err(BranchError::CannotMergeBranchesWithNoCommonAncestors(from, to).into());
        }

        // todo: remove the unwraps
        let common_ancestor = common_ancestor.unwrap();
        let common_tree = commit_mgr.get_commit_tree(common_ancestor, tree_mgr)?;

        let from_commit_hash = self.get_branch_head(&from)?.unwrap();
        let from_tree = commit_mgr.get_commit_tree(from_commit_hash, tree_mgr)?;

        let to_commit_hash = self.get_branch_head(&to)?.unwrap();
        let to_tree = commit_mgr.get_commit_tree(to_commit_hash, tree_mgr)?;

        let files_history = self.collect_files_history(
            common_tree, from_tree, to_tree);


        let mut conflicts: Vec<PathBuf> = Vec::new();
        for (rel_path, file_history) in files_history {
            let conflict = self.check_conflict(&file_history);

            match conflict {
                FileConflict::Present => conflicts.push(rel_path),
                FileConflict::None => {}
            }
        }

        if !conflicts.is_empty() {
            return Err(BranchError::MergeConflicts(conflicts).into());
        }

        Ok(())
    }


    /// Collects history of files in a common tree, "from" tree and "to" tree
    fn collect_files_history(
        &self,
        common_tree: Tree,
        from_tree: Tree,
        to_tree: Tree,
    ) -> BTreeMap<PathBuf, FileHistory> {
        let mut result = BTreeMap::new();

        for (rel_path, blob_hash) in common_tree.files {
            result.insert(rel_path, FileHistory::new(Some(blob_hash), None, None));
        }

        for (rel_path, blob_hash) in from_tree.files {
            if let Some(change) = result.get_mut(&rel_path) {
                change.from_blob_hash = Some(blob_hash);
            } else {
                result.insert(rel_path, FileHistory::new(None, Some(blob_hash), None));
            }
        }

        for (rel_path, blob_hash) in to_tree.files {
            if let Some(change) = result.get_mut(&rel_path) {
                change.to_blob_hash = Some(blob_hash);
            } else {
                result.insert(rel_path, FileHistory::new(None, None, Some(blob_hash)));
            }
        }

        result
    }

    /// Checks for a conflict given a [`FileHistory`]
    fn check_conflict(
        &self,
        file_history: &FileHistory
    ) -> FileConflict {
        let common_ancestor = &file_history.common_ancestor_blob_hash;
        let from = &file_history.from_blob_hash;
        let to = &file_history.to_blob_hash;

        if let Some(from) = from && let Some(to) = to
            && from != to {
            return FileConflict::Present;
        }

        FileConflict::None
    }
}


/// This struct contains tracks file blob hashes across three different trees:
/// common ancestor tree, from_tree and to_tree
#[derive(Debug, Clone, Ord, Eq, PartialEq, PartialOrd)]
struct FileHistory {
    common_ancestor_blob_hash: Option<String>,
    from_blob_hash: Option<String>,
    to_blob_hash: Option<String>,
}

impl FileHistory {
    fn new(
        common_ancestor_blob_hash: Option<String>,
        from_blob_hash: Option<String>,
        to_blob_hash: Option<String>
    ) -> Self {
        Self {
            common_ancestor_blob_hash,
            from_blob_hash,
            to_blob_hash,
        }
    }
}


/// This enum represents a file conflict type
enum FileConflict {
    None,
    Present
}
