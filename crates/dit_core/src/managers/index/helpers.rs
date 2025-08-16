use crate::managers::tree::TreeMgr;
use crate::managers::index::IndexMgr;
use crate::managers::branch::BranchMgr;
use crate::managers::commit::CommitMgr;
use crate::managers::ignore::IgnoreMgr;
use crate::models::{
    Change, DeletedFile,
    FileFingerprint, Index,
    IndexEntry, ModifiedFile,
    NewFile, UnchangedFile
};
use crate::helpers::{hash_file, read_to_string, write_to_file};
use crate::errors::{DitResult, StagingError};
use std::collections::{BTreeMap, HashMap};
use std::path::{Path, PathBuf};
use rayon::prelude::*;

/// Manage the index file
impl IndexMgr {
    /// Updates the index based on the index file
    pub(super) fn load(&mut self) -> DitResult<()> {
        let path = self.repo.index_file();
        let serialized = read_to_string(path)?;

        let index = if serialized.is_empty() {
            Index::default()
        } else {
            serde_json::from_str(&serialized)
                .map_err(|_| StagingError::DeserializationError)?
        };

        self.index = index;

        Ok(())
    }

    /// Updates the index file based on the current state
    pub(super) fn store(&self) -> DitResult<()> {
        let path = self.repo.index_file();

        let serialized = serde_json::to_string_pretty(&self.index)
            .map_err(|_| StagingError::SerializationError)?;

        write_to_file(path, serialized)
    }
}

/// Getters
impl IndexMgr {
    pub fn index(&self) -> &Index {
        &self.index
    }

    /// Returns all tracked changes
    pub fn get_all_tracked_changes(
        &self,
        tree_mgr: &TreeMgr,
        commit_mgr: &CommitMgr,
        branch_mgr: &BranchMgr,
    ) -> DitResult<BTreeMap<PathBuf, Change>> {
        self.index.files
            .par_iter()
            .filter_map(|(rel_path, entry)| {
                match self._get_tracked_change(rel_path, &Some(entry), tree_mgr, commit_mgr, branch_mgr) {
                    Ok(change) => match change {
                        Change::None | Change::Unchanged(_) => None,
                        _ => Some(Ok((rel_path.to_path_buf(), change)))
                    }

                    Err(e) => Some(Err(e))
                }
            })
            .collect::<DitResult<BTreeMap<PathBuf, Change>>>()
    }

    /// Returns all untracked changes
    pub fn get_all_untracked_changes(
       &self,
       ignore_mgr: &IgnoreMgr
    ) -> DitResult<BTreeMap<PathBuf, Change>> {
        let mut changed_files = BTreeMap::new(); // use BTreeMap for a sorted result
        let mut unchanged_file = HashMap::new();
        ignore_mgr.walk_dir_files(self.repo.repo_path(), |abs_path| {
            let rel_path = self.repo.rel_path(&abs_path)?;
            let change = self.get_untracked_change(&rel_path)?;
            if let Change::New(_) | Change::Modified(_) | Change::Deleted(_) = change {
                changed_files.insert(rel_path, change);
            } else {
                unchanged_file.insert(rel_path, change);
            }
            Ok(())
        })?;

        // Detect deleted files
        for (rel_path, IndexEntry { fp, hash }) in &self.index.files {
            if !changed_files.contains_key(rel_path)
                && !unchanged_file.contains_key(rel_path)
                && !ignore_mgr.is_ignored(rel_path)
            {
                changed_files.insert(rel_path.clone(), Change::Deleted(DeletedFile {
                    fp: fp.clone(), hash: hash.clone()
                }));
            }
        }

        Ok(changed_files)
    }

    /// Checks whether there are any tracked changes
    pub fn are_tracked_changes(
        &self,
        tree_mgr: &TreeMgr,
        commit_mgr: &CommitMgr,
        branch_mgr: &BranchMgr,
    ) -> DitResult<bool> {
        Ok(!self.get_all_tracked_changes(tree_mgr, commit_mgr, branch_mgr)?.is_empty())
    }

    /// Checks whether there are any untracked changes
    pub fn are_untracked_changes(
        &self,
        ignore_mgr: &IgnoreMgr,
    ) -> DitResult<bool> {
        Ok(!self.get_all_untracked_changes(ignore_mgr)?.is_empty())
    }

    /// Returns the untracked and tracked changes of a file \
    /// `result.0` - untracked changes \
    /// `result.1` - tracked changes
    pub fn identify_changes(
        &self,
        rel_path: &Path,
        tree_mgr: &TreeMgr,
        commit_mgr: &CommitMgr,
        branch_mgr: &BranchMgr,
    ) -> DitResult<(Change, Change)> {
        let in_index = self.index.files.get(rel_path);

        let untracked_change = self._get_untracked_change(rel_path, &in_index)?;
        let tracked_change = self._get_tracked_change(
            rel_path, &in_index, tree_mgr, commit_mgr, branch_mgr
        )?;

        Ok((untracked_change, tracked_change))
    }

    pub fn get_tracked_change(
        &self,
        rel_path: &Path,
        tree_mgr: &TreeMgr,
        commit_mgr: &CommitMgr,
        branch_mgr: &BranchMgr
    ) -> DitResult<Change> {
        let in_index = self.index.files.get(rel_path);
        self._get_tracked_change(rel_path, &in_index, tree_mgr, commit_mgr, branch_mgr)
    }

    pub fn get_untracked_change(&self, rel_path: &Path) -> DitResult<Change> {
        let in_index = self.index.files.get(rel_path);
        self._get_untracked_change(rel_path, &in_index)
    }
}


/// Private
impl IndexMgr {
    fn _get_tracked_change(
        &self,
        rel_path: &Path,
        in_index: &Option<&IndexEntry>,
        tree_mgr: &TreeMgr,
        commit_mgr: &CommitMgr,
        branch_mgr: &BranchMgr,
    ) -> DitResult<Change> {
        let in_tree = branch_mgr
            .get_head_tree(tree_mgr, commit_mgr)?
            .and_then(|mut t| t.index.files.remove(rel_path));

        let change = match in_tree {
            Some(in_tree) => match in_index {
                Some(in_index) => {
                    if in_tree.hash == in_index.hash {
                        Change::Unchanged(UnchangedFile {
                            hash: in_tree.hash,
                            fp: in_tree.fp
                        })
                    } else {
                        Change::Modified(ModifiedFile {
                            old_hash: in_tree.hash,
                            hash: in_index.hash.clone(),
                            old_fp: in_tree.fp,
                            fp: in_index.fp.clone(),
                        })
                    }
                }

                None => Change::Deleted(DeletedFile {
                    hash: in_tree.hash,
                    fp: in_tree.fp
                }),
            }

            None => match in_index {
                Some(in_index) => Change::New(NewFile {
                    hash: in_index.hash.clone(),
                    fp: in_index.fp.clone()
                }),
                None => Change::None,
            }
        };

        Ok(change)
    }

    fn _get_untracked_change(&self, rel_path: &Path, in_index: &Option<&IndexEntry>) -> DitResult<Change> {
        let abs_path = self.repo.abs_path_from_repo(rel_path, true)?;
        let exists = abs_path.is_file();

        let change = match in_index {
            Some(IndexEntry { hash, fp }) => {
                // If the current file exists, we will compare the fingerprints before hashing
                if exists {
                    let current_fp = FileFingerprint::from(&abs_path)?;
                    if *fp == current_fp {
                        Change::Unchanged(UnchangedFile {
                            hash: hash.clone(), fp: current_fp
                        })
                    } else {
                        let new_hash = hash_file(&abs_path)?;
                        Change::Modified(ModifiedFile {
                            old_hash: hash.clone(),
                            hash: new_hash,
                            old_fp: fp.clone(),
                            fp: current_fp,
                        })
                    }
                } else {
                    Change::Deleted(DeletedFile { hash: hash.clone(), fp: fp.clone() })
                }
            }

            None => {
                if exists {
                    let fp = FileFingerprint::from(&abs_path)?;
                    Change::New(NewFile { hash: hash_file(&abs_path)?, fp })
                } else {
                    Change::None
                }
            }
        };
        Ok(change)
    }
}
