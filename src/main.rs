use std::path::PathBuf;
use dit::trees::{TreeBuilder, TreeMgr};

fn main() -> std::io::Result<()> {
    let project = PathBuf::from(r"C:\Users\davit.baghdasaryan1\Coding\dit");

    let tree_mgr = TreeMgr::from_project(project)?;

    let mut builder = TreeBuilder::new();
    builder.add_file(r"C:\Users\davit.baghdasaryan1\Coding\dit\src\lib.rs")?;
    builder.add_file(r"C:\Users\davit.baghdasaryan1\Coding\dit\src\trees.rs")?;

    let tree_hash = tree_mgr.create_tree(&builder)?;

    println!("{:?}", tree_hash);

    println!("{:?}", tree_mgr.get_tree(tree_hash));

    Ok(())
}

