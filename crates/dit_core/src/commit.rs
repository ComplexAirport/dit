use crate::tree::TreeMgr;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::io;
use std::rc::Rc;
use std::time::SystemTime;
use crate::dit_project::DitProject;
use crate::stage::StagedFiles;

/// Manages the commits in our Dit version control system
pub struct CommitMgr {
    project: Rc<DitProject>,

    /// Represents the tree manager [`TreeMgr`]
    tree_mgr: TreeMgr,
}

/// Constructors
impl CommitMgr {
    pub fn from(project: Rc<DitProject>) -> io::Result<Self> {
        let tree_mgr = TreeMgr::from(project.clone())?;
        Ok(Self {
            project,
            tree_mgr,
        })
    }
}

/// Manage commits
impl CommitMgr {
    /// Creates a commit and returns the commit hash
    pub fn create_commit(
        &self,
        author: String,
        message: String,
        staged_files: &StagedFiles,
        parent_commit_hash: Option<String>,
    ) -> io::Result<String> {
        let tree_hash = self.tree_mgr.create_tree(staged_files)?;

        let timestamp = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs();

        let mut hasher = Sha256::new();
        hasher.update(&author);
        hasher.update(&message);
        hasher.update(timestamp.to_le_bytes());
        hasher.update(&tree_hash);
        hasher.update(parent_commit_hash.clone().unwrap_or(String::from('\0')));
        let commit_hash = format!("{:x}", hasher.finalize());

        let commit = Commit {
            author,
            message,
            timestamp,
            tree: tree_hash,
            parent: parent_commit_hash,
            hash: commit_hash.clone(),
        };

        self.write_commit(&commit)?;

        Ok(commit_hash)
    }
}

/// Private helper methods
impl CommitMgr {
    /// Writes the given commit to the commits directory
    fn write_commit(&self, commit: &Commit) -> io::Result<()> {
        let serialized = serde_json::to_string_pretty(&commit)?;
        let path = self.project.commits().join(&commit.hash);
        std::fs::write(path, serialized)?;
        Ok(())
    }

    /// Reads and returns a commit given the commit's hash
    fn load_commit(&self, hash: &str) -> io::Result<Commit> {
        let path = self.project.commits().join(hash);
        let serialized = std::fs::read_to_string(path)?;
        let commit: Commit = serde_json::from_str(&serialized)?;
        Ok(commit)
    }
}


/// Represents the commit object
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Commit {
    /// Represents the committer name and email address \
    /// Example: "Alice | alice@example.com"
    author: String,

    /// Represents the commit message \
    /// Example: "initial commit"
    message: String,

    /// Represents the commit time as a Unix timestamp - number of seconds
    /// since January 1, 1970 (UTC)
    timestamp: u64,

    /// Represents the tree hash of this commit
    tree: String,

    /// Represents the hash of the parent commit (the hash of the commit which preceded this commit)
    parent: Option<String>,

    /// Represents the commit hash
    #[serde(skip)]
    hash: String,
}
