use crate::managers::tree::TreeMgr;
use crate::models::{Index, Tree};
use crate::errors::DitResult;
use crate::helpers::DitHasher;

impl TreeMgr {
    /// Creates a tree from an index and returns the tree hash
    pub fn create_tree(
        &self,
        index: Index
    ) -> DitResult<String> {
        let mut hasher = DitHasher::new();
        for (rel_path, entry) in &index.files {
            hasher.update(rel_path.to_string_lossy().as_bytes());
            hasher.update(entry.hash.as_bytes());
        }
        let hash = hasher.finalize_string();

        let tree = Tree { index, hash: hash.clone(), };

        self.write_tree(&tree)?;

        Ok(hash)
    }
}
