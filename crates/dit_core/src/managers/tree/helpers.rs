use crate::errors::DitResult;
use crate::managers::tree::TreeMgr;
use crate::models::Tree;


/// Manage tree files
impl TreeMgr {
    /// Reads and returns a tree from the tree's hash
    pub fn get_tree(&self, tree_hash: String) -> DitResult<Tree> {
        let path = self.repo.trees().join(tree_hash);

        Tree::read_from(&path)
    }
    
    /// Writes the tree to the trees directory
    pub(super) fn write_tree(&self, tree: &Tree) -> DitResult<()> {
        let path = self.repo.trees().join(&tree.hash);
        
        tree.write_to(&path)
    }
}
