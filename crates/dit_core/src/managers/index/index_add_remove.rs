use crate::managers::index::IndexMgr;
use crate::managers::branch::BranchMgr;
use crate::managers::commit::CommitMgr;
use crate::managers::tree::TreeMgr;
use crate::managers::blob::BlobMgr;
use crate::models::{Change, DeletedFile, IndexEntry, ModifiedFile, NewFile};
use crate::errors::DitResult;
use std::path::Path;

impl IndexMgr {
    /// Adds files in their current state to the index
    pub fn add_files(
        &mut self,
        paths: impl IntoIterator<Item = impl AsRef<Path>>,
        blob_mgr: &BlobMgr
    ) -> DitResult<()> {
        for file_path in paths {
            let file_path = file_path.as_ref();
            let rel_path = self.repo.rel_path(file_path)?;
            let untracked = self.get_untracked_change(&rel_path)?;
            match &untracked {
                Change::New(NewFile { hash, fp })
                | Change::Modified(ModifiedFile { hash, fp, .. }) => {
                    blob_mgr.create_blob_with_hash(file_path, hash.clone())?;
                    self.index.files.insert(rel_path, IndexEntry {
                        hash: hash.clone(),
                        fp: fp.clone()
                    });
                }

                Change::Deleted(_) => {
                    self.index.files.remove(&rel_path);
                }

                _ => {}
            }
        }

        self.store()
    }

    /// Unstages files
    pub fn unstage_files(
        &mut self,
        paths: impl IntoIterator<Item = impl AsRef<Path>>,
        tree_mgr: &TreeMgr,
        commit_mgr: &CommitMgr,
        branch_mgr: &BranchMgr
    ) -> DitResult<()> {
        for file_path in paths {
            let file_path = file_path.as_ref();
            let rel_path = self.repo.abs_path_from_cwd(file_path, false)?;

            let (_, tracked) = self.identify_changes(&rel_path, tree_mgr, commit_mgr, branch_mgr)?;
            match tracked {
                Change::New(_) => {
                    self.index.files.remove(&rel_path);
                },
                Change::Modified(ModifiedFile { old_fp, old_hash, .. }) => {
                    self.index.files.insert(rel_path, IndexEntry { hash: old_hash, fp: old_fp });
                }
                Change::Deleted(DeletedFile { fp, hash }) => {
                    self.index.files.insert(rel_path, IndexEntry { fp, hash });
                }

                _ => {}
            };
        }

        self.store()
    }

    /// Unstaged all tracked changes
    pub fn unstage_all(
        &mut self,
        tree_mgr: &TreeMgr,
        commit_mgr: &CommitMgr,
        branch_mgr: &BranchMgr
    ) -> DitResult<()> {
        let tracked_changes = self.get_all_tracked_changes(tree_mgr, commit_mgr, branch_mgr)?;
        let tracked_paths = tracked_changes.keys();
        self.unstage_files(tracked_paths, tree_mgr, commit_mgr, branch_mgr)
    }
}
