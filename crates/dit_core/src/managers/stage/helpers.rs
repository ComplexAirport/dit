use crate::managers::tree::TreeMgr;
use crate::managers::stage::StageMgr;
use crate::managers::branch::BranchMgr;
use crate::managers::commit::CommitMgr;
use crate::models::{NewFile, ChangeType, ModifiedFile, Stage, FileFingerprint, UnchangedFile};
use crate::helpers::{hash_file, path_to_string, read_to_string, write_to_file};
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
    ) -> DitResult<(ChangeType /* Untracked change */, Option<ChangeType> /* tracked change */)>
    {
        let abs_path = self.repo.abs_path_from_repo(rel_path, true)?;

        // Calculate the current fingerprint of the file
        let fp = FileFingerprint::from(&abs_path)?;

        // First, check if the path is tracked
        let in_stage = self.stage.files.get(rel_path);

        // In case the file is tracked, we can compare the metadata and see whether it differs
        // if the metadata differs, we can calculate the new hash
        let current_hash = match in_stage {
            Some(change) => match change {
                ChangeType::Deleted => None,

                ChangeType::Unchanged(UnchangedFile { fingerprint, hash })
                | ChangeType::Modified(ModifiedFile { fingerprint, hash, .. })
                | ChangeType::New(NewFile { fingerprint, hash })
                if *fingerprint == fp
                    => Some(hash.clone()),

                _ => Some(hash_file(&abs_path)?)
            }

            _other if abs_path.is_file() => Some(hash_file(&abs_path)?),

            _ => None,
        };

        // Access the path from the tree
        let in_tree = branch_mgr
            .get_head_tree(tree_mgr, commit_mgr)?
            .and_then(|mut t| t.files.remove(rel_path));

        // Perform a three-way comparison of the file in the current filesystem,
        // the tree and in the stage
        let untracked_change = match current_hash {
            None => match in_stage {
                None => match in_tree {
                    None => return Err(StagingError::FileNotFound(path_to_string(rel_path)).into()),
                    Some(_) => ChangeType::Deleted
                }

                Some(_) => ChangeType::Deleted
            }
            Some(hash) => match in_stage {
                None => match in_tree {
                    None => ChangeType::New(NewFile { hash, fingerprint: fp }),
                    Some(old_hash) =>
                        if old_hash == hash {
                            ChangeType::Unchanged(UnchangedFile { hash, fingerprint: fp })
                        } else {
                            ChangeType::Modified(ModifiedFile { old_hash, hash, fingerprint: fp })
                        }
                }

                Some(change) => match change {
                    ChangeType::Unchanged(u) => ChangeType::Unchanged(u.clone()),
                    ChangeType::Deleted => ChangeType::New(NewFile { hash, fingerprint: fp }),
                    ChangeType::Modified(file) => {
                        if file.hash == hash {
                            ChangeType::Unchanged(UnchangedFile { hash, fingerprint: fp })
                        } else {
                            ChangeType::Modified(ModifiedFile {
                                old_hash: file.old_hash.clone(),
                                hash,
                                fingerprint: fp,
                            })
                        }
                    }
                    ChangeType::New(file) => {
                        if file.hash == hash {
                            ChangeType::Unchanged(UnchangedFile { hash, fingerprint: fp})
                        } else {
                            ChangeType::Modified(ModifiedFile {
                                old_hash: file.hash.clone(),
                                hash,
                                fingerprint: fp
                            })
                        }
                    }
                }
            }
        };

        let tracked_change = match in_stage {
            Some(change) if !matches!(change, ChangeType::Unchanged(_)) => Some(change.clone()),
            _ => None
        };

        Ok((untracked_change, tracked_change))
    }
}