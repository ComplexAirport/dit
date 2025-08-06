use crate::Dit;
use crate::api_models::{ChangeType, Status};
use crate::helpers::calculate_hash;
use crate::errors::DitResult;
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;


impl Dit {
    /// Returns the current dit status (tracked/untracked files, etc.)
    pub fn get_status(&self) -> DitResult<Status> {
        let stage_mgr = self.stage_mgr.borrow();

        // First, get the list of the staged files
        let mut staged_files = stage_mgr.get_stage().files.clone();

        // Then, get the previous tree files
        let tree = self.branch_mgr.borrow().get_head_tree(
            &self.tree_mgr.borrow(),
            &self.commit_mgr.borrow(),
        )?;
        let mut tree_files = tree.map(|t| t.files)
            .unwrap_or_default();

        let mut status = Status::new();
        for entry in WalkDir::new(self.repo.repo_path())
            .min_depth(1)
            .into_iter()
            .filter_map(Result::ok)
            .filter(|e| e.file_type().is_file())
        {
            let abs_path = entry.path().to_path_buf();
            let rel_path = self.repo.get_relative_path(&abs_path)?;

            self.three_way_comparison(
                rel_path,
                &mut tree_files,
                &mut staged_files,
                &mut status
            )?;
        }

        // Now check for files left in the tree and in the stage. These files were deleted
        for rel_path in tree_files.keys() {
            status.add_untracked(rel_path.clone(), ChangeType::Deleted);
        }

        for rel_path in staged_files.keys() {
            status.add_tracked(rel_path.clone(), ChangeType::Deleted);
        }


        Ok(status)
    }
}

/// Private
impl Dit {
    /// Compares a file in HEAD tree, stage and in current status, and makes a corresponding
    /// change in the [`Status`]
    fn three_way_comparison(
        &self,
        rel_path: PathBuf,
        tree_files: &mut BTreeMap<PathBuf, String>,
        staged_files: &mut BTreeMap<PathBuf, PathBuf>,
        status: &mut Status,
    ) -> DitResult<()> {
        let in_stage = staged_files.remove(&rel_path);
        let in_tree = tree_files.remove(&rel_path);

        match &in_tree {
            Some(tree_blob_hash) => match in_stage {
                // In Tree, in Stage
                Some(stage_blob_path) => {
                    self._in_stage_in_tree(rel_path, tree_blob_hash, &stage_blob_path, status)?;
                }
                // In Tree, not in Stage
                None => {
                    self._not_in_stage_in_tree(rel_path, tree_blob_hash, status)?;
                }
            }

            None => match in_stage {
                // In Stage, not in Tree
                Some(stage_blob_path) => {
                    self._in_stage_not_in_tree(rel_path, stage_blob_path, status)?;
                }
                // Not in Tree, not in Stage
                None => {
                    self._not_in_stage_not_in_tree(rel_path, status);
                }
            }
        }

        Ok(())
    }
}


/// Private
impl Dit {
    fn _in_stage_in_tree(
        &self,
        rel_path: PathBuf,
        tree_blob_hash: &String,
        stage_blob_path: &Path,
        status: &mut Status,
    ) -> DitResult<()> {
        let stage_blob_hash = stage_blob_path
            .file_name()
            .unwrap_or_default()
            .to_string_lossy()
            .to_string();

        let new_blob_hash = calculate_hash(&rel_path)?;

        if *tree_blob_hash != stage_blob_hash {
            status.add_tracked(rel_path.clone(), ChangeType::Modified);
        }

        if new_blob_hash != stage_blob_hash {
            status.add_untracked(rel_path.clone(), ChangeType::Modified);
        }

        if new_blob_hash == *tree_blob_hash && new_blob_hash == stage_blob_hash {
            status.add_unchanged(rel_path);
        }

        Ok(())
    }

    fn _in_stage_not_in_tree(
        &self,
        rel_path: PathBuf,
        stage_blob_path: PathBuf,
        status: &mut Status,
    ) -> DitResult<()> {
        let stage_blob_hash = stage_blob_path
            .file_name()
            .unwrap_or_default()
            .to_string_lossy()
            .to_string();

        let new_blob_hash = calculate_hash(&rel_path)?;

        if stage_blob_hash != new_blob_hash {
            status.add_tracked(rel_path.clone(), ChangeType::New);
            status.add_untracked(rel_path, ChangeType::Modified);
        } else {
            status.add_tracked(rel_path, ChangeType::New);
        }

        Ok(())
    }

    fn _not_in_stage_in_tree(
        &self,
        rel_path: PathBuf,
        tree_blob_hash: &String,
        status: &mut Status,
    ) -> DitResult<()> {
        let new_blob_hash = calculate_hash(&rel_path)?;

        if new_blob_hash != *tree_blob_hash {
            status.add_untracked(rel_path, ChangeType::Modified);
        } else {
            status.add_unchanged(rel_path);
        }

        Ok(())
    }

    fn _not_in_stage_not_in_tree(
        &self,
        rel_path: PathBuf,
        status: &mut Status,
    ) {
        status.add_untracked(rel_path, ChangeType::New);
    }
}
