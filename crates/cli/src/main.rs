use dit_core::dit::Dit;

fn main() -> std::io::Result<()> {
    let mut dit = Dit::from_project(r"C:\Users\davit.baghdasaryan1\Coding\dit")?;

    dit.stage_file(r"C:\Users\davit.baghdasaryan1\Coding\dit\crates\dit_core\src\dit.rs")?;
    // dit.stage_file(r"C:\Users\davit.baghdasaryan1\Coding\dit\crates\dit_core\src\trees.rs")?;

    dit.commit("Davit Baghdasaryan", "commit 3")?;

    Ok(())
}

