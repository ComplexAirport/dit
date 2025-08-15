use crate::managers::stage::StageMgr;
use crate::managers::branch::BranchMgr;
use crate::managers::commit::CommitMgr;
use crate::managers::tree::TreeMgr;
use crate::managers::blob::BlobMgr;
use crate::models::{ChangeType, ModifiedFile, NewFile};
use crate::errors::DitResult;
use crate::helpers::remove_file_if_exists;
use std::path::Path;

impl StageMgr {
    /// Stages a file based on its path. IMPORTANT: files in the
    /// stage folder have their names as their content hashes.
    /// This way, the blob hash doesn't have to be recomputed
    /// when the file is commited
    pub fn stage_files(
        &mut self,
        paths: impl IntoIterator<Item = impl AsRef<Path>>,
        blob_mgr: &BlobMgr,
        tree_mgr: &TreeMgr,
        commit_mgr: &CommitMgr,
        branch_mgr: &BranchMgr
    ) -> DitResult<()> {
        for file_path in paths {
            let file_path = file_path.as_ref();
            let rel_path = self.repo.rel_path(file_path)?;
            let (change, _) = self.get_changes(&rel_path, tree_mgr, commit_mgr, branch_mgr)?;
            match &change {
                ChangeType::New(NewFile { hash, .. }) => {
                    blob_mgr.create_temp_blob_with_hash(file_path, hash.clone())?;

                }
                ChangeType::Modified(ModifiedFile { hash: new_hash, .. }) => {
                    blob_mgr.create_temp_blob_with_hash(file_path, new_hash.clone())?;
                }
                _ => {}
            }

            if !matches!(change, ChangeType::Unchanged(_)) {
                self.stage.files.insert(rel_path, change);
            }
        }

        self.update_stage_file()
    }

    /// Unstages a file based on its path
    pub fn unstage_files(&mut self, paths: impl IntoIterator<Item = impl AsRef<Path>>)
        -> DitResult<()> {
        for file_path in paths {
            let file_path = file_path.as_ref();
            let rel_path = self.repo.abs_path_from_cwd(file_path, false)?;

            let staged_path = self.stage.files.remove(&rel_path);

            if let Some(change) = staged_path {
                match change {
                    ChangeType::New(file) => {
                        let temp_blob_path = self.repo.stage().join(file.hash);
                        remove_file_if_exists(&temp_blob_path)?;
                    }

                    ChangeType::Modified(file) => {
                        let temp_blob_path = self.repo.stage().join(file.hash);
                        remove_file_if_exists(&temp_blob_path)?;
                    }

                    _ => {}
                }
            }
        }

        self.update_stage_file()
    }

    /// Clears the [`STAGE_FILE`] and inner cache
    ///
    /// [`STAGE_FILE`]: crate::project_structure::STAGE_FILE
    pub fn clear_stage_file(&mut self) -> DitResult<()> {
        self.stage.files.retain(|_, change| matches!(change, ChangeType::Unchanged(_)));
        self.update_stage_file()
    }

    /// Clears the [`STAGE_FILE`] and inner cache and removes the temporary blobs
    pub fn clear_stage_all(&mut self, blob_mgr: &BlobMgr) -> DitResult<()> {
        self.stage.files.values()
            .try_for_each(|change| {
                match change {
                    ChangeType::Modified(ModifiedFile { hash: new_hash, .. }) =>
                        blob_mgr.remove_temp_blob(new_hash.clone()),

                    ChangeType::New(NewFile { hash, .. }) =>
                        blob_mgr.remove_temp_blob(hash.clone()),

                    _ => Ok(())
                }
            })?;

        self.clear_stage_file()
    }
}
