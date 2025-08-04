use crate::managers::commit::CommitMgr;
use std::collections::{HashSet, VecDeque};

/// Iterates through ancestors of a commit using breadth-first search
pub struct CommitBfsIterator<'a> {
    visited: HashSet<String>,
    queue: VecDeque<String>,
    commit_mgr: &'a CommitMgr,
}

impl<'a> CommitBfsIterator<'a> {
    pub fn new<S: Into<String>>(
        start_commit: S,
        commit_mgr: &'a CommitMgr
    ) -> Self
    {
        Self {
            visited: HashSet::new(),
            queue: VecDeque::from([start_commit.into()]),
            commit_mgr,
        }
    }
}

impl<'a> Iterator for CommitBfsIterator<'a> {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(commit_hash) = self.queue.pop_front() {
            if self.visited.contains(&commit_hash) {
                continue;
            }
            self.visited.insert(commit_hash.clone());

            let commit = self.commit_mgr.get_commit(&commit_hash);
            if commit.is_err() {
                return None;
            }
            let commit = commit.unwrap();

            for parent in &commit.parents {
                if !self.visited.contains(parent) {
                    self.queue.push_back(parent.clone());
                }
            }

            return Some(commit_hash)
        }

        None
    }
}
