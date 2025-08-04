use crate::managers::commit::commit_iterator::CommitBfsIterator;
use crate::managers::commit::CommitMgr;
use crate::managers::tree::TreeMgr;
use crate::models::{Commit, Tree};
use crate::errors::DitResult;

/// Load/write to the commits directory
impl CommitMgr {
    /// Writes the given commit to the commits directory
    pub(super) fn write_commit(&self, commit: &Commit) -> DitResult<()> {
        let path = self.repo.commits().join(&commit.hash);
        commit.write_to(path)?;
        Ok(())
    }

    /// Reads and returns a commit given the commit's hash
    pub(super) fn load_commit<S: Into<String>>(&self, hash: S) -> DitResult<Commit> {
        let hash = hash.into();
        let path = self.repo.commits().join(&hash);

        let mut commit = Commit::read_from(path)?;

        commit.hash = hash;

        Ok(commit)
    }
}


/// Getters
impl CommitMgr {
    /// Returns a commit by hash
    pub fn get_commit<S: Into<String>>(&self, hash: S) -> DitResult<Commit> {
        self.load_commit(hash)
    }


    /// Returns the tree of a commit by commit hash
    pub fn get_commit_tree<S: Into<String>>(
        &self,
        hash: S,
        tree_mgr: &TreeMgr
    ) -> DitResult<Tree> {
        let commit = self.get_commit(hash)?;
        let tree = tree_mgr.get_tree(commit.tree)?;
        Ok(tree)
    }

    /// Returns the parent commit hash(es) of a given commit
    pub fn get_parents<S: Into<String>>(&self, hash: S) -> DitResult<Vec<String>> {
        let hash = hash.into();
        let path = self.repo.commits().join(&hash);
        let commit = Commit::read_from(path)?;
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


    // /// Tries to find a common ancestor for two commits
    // pub fn common_ancestor<S1, S2>(&self, a: S1, b: S2) -> DitResult<Option<String>>
    // where
    //     S1: Into<String>,
    //     S2: Into<String>,
    // {
    //     let mut visited_a = HashSet::new();
    //     let mut visited_b = HashSet::new();
    //
    //     let mut queue_a = VecDeque::from([a.into()]);
    //     let mut queue_b = VecDeque::from([b.into()]);
    //
    //     while !queue_a.is_empty() && !queue_b.is_empty() {
    //         if let Some(current_a) = queue_a.pop_front() {
    //             if visited_b.contains(&current_a) {
    //                 return Ok(Some(current_a));
    //             }
    //
    //             if visited_a.insert(current_a.clone()) {
    //                 let parent = self.get_parent(current_a)?;
    //                 if let Some(parent) = parent {
    //                     queue_a.push_back(parent);
    //                 }
    //             }
    //         }
    //
    //         if let Some(current_b) = queue_b.pop_front() {
    //             if visited_a.contains(&current_b) {
    //                 return Ok(Some(current_b));
    //             }
    //
    //             if visited_b.insert(current_b.clone()) {
    //                 let parent = self.get_parent(current_b)?;
    //                 if let Some(parent) = parent {
    //                     queue_b.push_back(parent);
    //                 }
    //             }
    //         }
    //     }
    //
    //     Ok(None)
    // }
}
