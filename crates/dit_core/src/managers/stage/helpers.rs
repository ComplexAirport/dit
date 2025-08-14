use crate::managers::tree::TreeMgr;
use crate::managers::stage::StageMgr;
use crate::managers::branch::BranchMgr;
use crate::managers::commit::CommitMgr;
use crate::models::{NewFile, ChangeType, ModifiedFile, Stage};
use crate::helpers::{hash_file, read_to_string, write_to_file};
use crate::errors::{DitResult, StagingError};
use std::path::Path;

/// Manage the stage file
impl StageMgr {
    /// Updates staged files stored in self based on the data in the [`STAGE_FILE`]
    ///
    /// [`STAGE_FILE`]: crate::project_structure::STAGE_FILE
    pub(super) fn load_stage_file(&mut self) -> DitResult<()> {
        let path = self.repo.stage_file();
        let serialized = read_to_string(path)?;

        let staged_files = if serialized.is_empty() {
            Stage::default()
        } else {
            serde_json::from_str(&serialized)
                .map_err(|_| StagingError::DeserializationError)?
        };

        self.stage = staged_files;

        Ok(())
    }

    /// Updates the data in the [`STAGE_FILE`] based on staged files stored in self
    ///
    /// [`STAGE_FILE`]: crate::project_structure::STAGE_FILE
    pub(super) fn update_stage_file(&self) -> DitResult<()> {
        let path = self.repo.stage_file();

        let serialized = serde_json::to_string_pretty(&self.stage)
            .map_err(|_| StagingError::SerializationError)?;

        write_to_file(path, serialized)
    }
}

/// Getters
impl StageMgr {
    pub fn is_staged(&self) -> bool {
        !self.stage.files.is_empty()
    }

    pub fn get_stage(&self) -> &Stage {
        &self.stage
    }

    /// Returns the untracked and tracked changes of a file \
    /// The first element in the result tuple represents the untracked changes,
    /// and the second element represents the tracked changes
    pub fn get_changes(
        &self,
        rel_path: &Path,
        tree_mgr: &TreeMgr,
        commit_mgr: &CommitMgr,
        branch_mgr: &BranchMgr,
    ) -> DitResult<(ChangeType /* Untracked change */, ChangeType /* Tracked change */)>
    {
        let abs_path = self.repo.abs_path_from_repo(rel_path, true)?;

        let current_hash = if abs_path.is_file() {
            Some(hash_file(&abs_path)?)
        } else {
            None
        };

        let in_tree = branch_mgr
            .get_head_tree(tree_mgr, commit_mgr)?
            .and_then(|mut t| t.files.remove(rel_path));

        let in_stage = self.stage.files.get(rel_path);

        let untracked_change = self.three_way_comparison(current_hash, in_tree, in_stage);

        let tracked_change = match in_stage {
            None => ChangeType::Unchanged,
            Some(change) => change.clone()
        };

        Ok((untracked_change, tracked_change))
    }
}

/// Private
impl StageMgr {
    /// Returns a possible untracked change of a file
    fn three_way_comparison(
        &self,
        current: Option<String>,
        in_tree: Option<String>,
        in_stage: Option<&ChangeType>
    ) -> ChangeType {
        match current {
            None => match in_stage {
                None => match in_tree {
                    None => ChangeType::Unchanged,
                    Some(_) => ChangeType::Deleted
                }

                Some(_) => ChangeType::Deleted
            }
            Some(hash) => match in_stage {
                None => match in_tree {
                    None => ChangeType::New(NewFile { hash }),
                    Some(old_hash) =>
                        if old_hash == hash {
                            ChangeType::Unchanged
                        } else {
                            ChangeType::Modified(ModifiedFile { old_hash, new_hash: hash, })
                        }
                }

                Some(change) => match change {
                    ChangeType::Unchanged => ChangeType::Unchanged,
                    ChangeType::Deleted => ChangeType::New(NewFile { hash }),
                    ChangeType::Modified(file) => {
                        if file.new_hash == hash {
                            ChangeType::Unchanged
                        } else {
                            ChangeType::Modified(ModifiedFile {
                                old_hash: file.old_hash.clone(),
                                new_hash: hash,
                            })
                        }
                    }
                    ChangeType::New(file) => {
                        if file.hash == hash {
                            ChangeType::Unchanged
                        } else {
                            ChangeType::Modified(ModifiedFile {
                                old_hash: file.hash.clone(),
                                new_hash: hash,
                            })
                        }
                    }
                }
            }
        }
    }
}
