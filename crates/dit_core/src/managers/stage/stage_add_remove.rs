use crate::managers::stage::StageMgr;
use crate::managers::branch::BranchMgr;
use crate::managers::commit::CommitMgr;
use crate::managers::tree::TreeMgr;
use crate::errors::DitResult;
use crate::helpers::{copy_file, remove_file};
use crate::models::{ChangeType, ModifiedFile, NewFile};
use std::path::Path;

impl StageMgr {
    /// Stages a file based on its path. IMPORTANT: files in the
    /// stage folder have their names as their content hashes.
    /// This way, the blob hash doesn't have to be recomputed
    /// when the file is commited
    pub fn stage_file<P: AsRef<Path>>(
        &mut self,
        file_path: P,
        tree_mgr: &TreeMgr,
        commit_mgr: &CommitMgr,
        branch_mgr: &BranchMgr
    ) -> DitResult<()> {
        let rel_path = self.repo.rel_path(&file_path)?;
        let (change, _) = self.get_changes(&rel_path, tree_mgr, commit_mgr, branch_mgr)?;

        match &change {
            ChangeType::New(NewFile { hash, .. }) => {
                let target_dir = self.repo.stage().join(hash);
                copy_file(file_path, &target_dir)?;
            }
            ChangeType::Modified(ModifiedFile { new_hash, .. }) => {
                let target_dir = self.repo.stage().join(new_hash);
                copy_file(file_path, &target_dir)?;
            }

            _ => {}
        }

        if !matches!(change, ChangeType::Unchanged) {
            self.stage.files.insert(rel_path, change);
            self.update_stage_file()?;
        }

        Ok(())
    }

    /// Unstages a file based on its path
    pub fn unstage_file<P: AsRef<Path>>(&mut self, file_path: P) -> DitResult<()> {
        let file_path = file_path.as_ref();
        let rel_path = self.repo.abs_path_from_cwd(file_path, false)?;

        let staged_path = self.stage.files.remove(&rel_path);

        if let Some(change) = staged_path {
            match change {
                ChangeType::New(file) => {
                    let temp_blob_path = self.repo.stage().join(file.hash);
                    remove_file(&temp_blob_path)?;
                }

                ChangeType::Modified(file) => {
                    let temp_blob_path = self.repo.stage().join(file.new_hash);
                    remove_file(&temp_blob_path)?;
                }

                ChangeType::Deleted(_) => {}
                ChangeType::Unchanged => {}
            }
        }

        self.update_stage_file()?;

        Ok(())
    }

    /// Clears all staged files and clears the [`STAGE_FILE`]
    ///
    /// - `remove_files` - specifies whether to remove files from the filesystem or only
    ///   update the inner state and the stage file
    ///
    /// [`STAGE_FILE`]: crate::project_structure::STAGE_FILE
    pub fn clear_stage(&mut self, remove_files: bool) -> DitResult<()> {
        if remove_files {
            for change in self.stage.files.values() {
                match change {
                    ChangeType::Modified(ModifiedFile { new_hash, .. }) => {
                        let temp_blob_path = self.repo.blobs().join(new_hash);
                        remove_file(&temp_blob_path)?;
                    }

                    ChangeType::New(NewFile { hash, .. }) => {
                        let temp_blob_path = self.repo.blobs().join(hash);
                        remove_file(&temp_blob_path)?;
                    }

                    _ => {}
                }
            }
        }
        self.stage.files.clear();
        self.update_stage_file()?;
        Ok(())
    }
}