use crate::managers::commit::commit_iterator::CommitBfsIterator;
use crate::managers::commit::CommitMgr;
use crate::managers::tree::TreeMgr;
use crate::models::{Commit, Tree};
use crate::errors::DitResult;
use crate::helpers::DitModel;

/// Load/write to the commits directory
impl CommitMgr {
    /// Writes the given commit to the commits directory
    pub(super) fn write_commit(&self, commit: &Commit) -> DitResult<()> {
        let path = self.repo.commits().join(&commit.hash);

        commit.serialize_to(&path)
    }

    /// Reads and returns a commit given the commit's hash
    pub fn get_commit<S: Into<String>>(&self, hash: S) -> DitResult<Commit> {
        let hash = hash.into();
        let path = self.repo.commits().join(&hash);

        let mut commit = Commit::deserialize_from(&path)?;


        commit.hash = hash;

        Ok(commit)
    }
}


/// Getters
impl CommitMgr {
    /// Returns the tree of a commit by commit hash
    pub fn get_commit_tree<S: Into<String>>(
        &self,
        hash: S,
        tree_mgr: &TreeMgr
    ) -> DitResult<Tree> {
        let commit = self.get_commit(hash)?;

        tree_mgr.get_tree(commit.tree)
    }

    /// Returns the parent commit hash(es) of a given commit
    pub fn get_parents<S: Into<String>>(&self, hash: S) -> DitResult<Vec<String>> {
        let hash = hash.into();
        let path = self.repo.commits().join(&hash);
        let commit = Commit::deserialize_from(&path)?;
        Ok(commit.parents)
    }


    /// Checks whether a commit is an ancestor to another commit
    pub fn is_ancestor<S1, S2>(&self, ancestor: S1, child: S2) -> DitResult<bool>
    where
        S1: Into<String>,
        S2: Into<String>,
    {
        let ancestor = ancestor.into();
        let child = child.into();

        let commit_iterator = CommitBfsIterator::new(child, self);
        for commit in commit_iterator {
            if commit == ancestor {
                return Ok(true);
            }
        }

        Ok(false)
    }
}
