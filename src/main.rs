use std::path::PathBuf;
use dit::blobs::BlobMgr;

fn main() -> std::io::Result<()> {
    let root = PathBuf::from(".dit");
    std::fs::create_dir_all(&root).expect("something failed");

    let blobmgr = BlobMgr::from_dit_root(root)?;

    blobmgr.add_file(r"D:\Programming\Rust\dit\src\main.rs")?;

    Ok(())
}

