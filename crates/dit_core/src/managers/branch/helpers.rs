use crate::errors::DitResult;
use crate::helpers::{read_to_string, write_to_file};
use crate::managers::branch::BranchMgr;
use std::path::PathBuf;
use crate::managers::commit::CommitMgr;
use crate::managers::tree::TreeMgr;
use crate::models::Tree;

/// Load/store from/to the [`HEAD_FILE`]
///
/// [`HEAD_FILE`]: crate::project_structure::HEAD_FILE
impl BranchMgr {
    /// Loads the current branch and(or) commit based on [`HEAD_FILE`]
    ///
    /// [`HEAD_FILE`]: crate::project_structure::HEAD_FILE
    pub(super) fn load(&mut self) -> DitResult<()> {
        let path = self.repo.head_file();
        let head = read_to_string(path)?;

        // if the head starts with ":", then it references a commit and not a branch
        if let Some(head) = head.strip_prefix(':') {
            self.curr_commit = Some(head.to_string());
            self.curr_branch = None;
        } else if head.is_empty() {
            self.curr_branch = None;
            self.curr_commit = None;
        } else {
            let path = self.repo.branches().join(&head);
            let commit = read_to_string(&path)?;
            if commit.is_empty() {
                self.curr_commit = None;
            } else {
                self.curr_commit = Some(commit);
            }
            self.curr_branch = Some(head);
        }

        Ok(())
    }

    /// Updates the [`HEAD_FILE`] based on the current branch and(or) commit stored in self
    ///
    /// [`HEAD_FILE`]: crate::project_structure::HEAD_FILE
    pub(super) fn store(&mut self) -> DitResult<()> {
        let head_file = self.repo.head_file();

        if let Some(curr_branch) = &self.curr_branch {
            write_to_file(head_file, curr_branch)?;
            let branch_file = self.repo.branches().join(curr_branch);
            match &self.curr_commit {
                Some(curr_commit) => write_to_file(&branch_file, curr_commit)?,
                None => write_to_file(&branch_file, "")?,
            }
        } else {
            match &self.curr_commit {
                Some(head) => write_to_file(head_file, format!(":{head}"))?,
                None => write_to_file(head_file, "")?,
            }
        }

        Ok(())
    }
}


/// Branch head operations
impl BranchMgr {
    /// Sets the current (head) branch to a new value
    pub fn set_current_branch<S: AsRef<str>>(&mut self, branch: S) -> DitResult<()> {
        let branch = branch.as_ref();
        self.curr_branch = Some(branch.to_string());
        self.store()?;
        Ok(())
    }

    /// Returns the name of the current (head) branch
    pub fn get_current_branch(&self) -> Option<&String> {
        self.curr_branch.as_ref()
    }

    /// Sets the head of a given branch to a given commit
    pub fn set_branch_head<S1, S2>(&mut self, branch: S1, commit: S2)
        -> DitResult<()>
    where
        S1: AsRef<str>,
        S2: AsRef<str>,
    {
        let branch = branch.as_ref();
        let commit = commit.as_ref();

        let branch_file = self.repo.branches().join(&branch);

        if branch_file.is_file() {
            write_to_file(&branch_file, commit)?;
        }
        Ok(())
    }

    /// Returns the head commit of a given branch
    pub(super) fn get_branch_head<S: AsRef<str>>(&self, name: S) -> DitResult<Option<String>> {
        let (exists, path) = self.find_branch(name);

        if exists {
            let content = read_to_string(&path)?;
            if content.is_empty() {
                Ok(None)
            } else {
                Ok(Some(content))
            }
        } else {
            Ok(None)
        }
    }
}


/// Commit head operations
impl BranchMgr {
    /// Sets the current (head) commit to a new value
    pub fn set_head_commit<S: AsRef<str>>(&mut self, commit: S) -> DitResult<()> {
        let commit = commit.as_ref();
        self.curr_commit = Some(commit.to_string());
        self.store()?;
        Ok(())
    }

    /// Returns the hash of the current commit
    pub fn get_head_commit(&self) -> Option<&String> { self.curr_commit.as_ref() }

    /// Return the tree of the current commit
    pub fn get_head_tree(&self, tree_mgr: &TreeMgr, commit_mgr: &CommitMgr) -> DitResult<Option<Tree>> {
        let head_commit = self.get_head_commit();

        match head_commit {
            None => Ok(None),
            Some(head_commit) => {
                let tree = commit_mgr.get_commit_tree(head_commit, tree_mgr)?;
                Ok(Some(tree))
            }
        }
    }
}


/// Branch getters
impl BranchMgr {
    /// Returns a bool indicating whether the branch exists or not and
    /// the path to that branch file
    pub(super) fn find_branch<S: AsRef<str>>(&self, name: S) -> (bool, PathBuf) {
        let name = name.as_ref();
        let path = self.repo.branches().join(name);

        (path.exists(), path)
    }
}
