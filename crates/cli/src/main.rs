use dit_core::dit::Dit;

fn main() -> std::io::Result<()> {
    let mut dit = Dit::from(r"C:\Users\davit.baghdasaryan1\Coding\dit")?;

    // dit.stage(r"C:\Users\davit.baghdasaryan1\Coding\dit\crates\dit_core\src\dit.rs")?;
    dit.stage(r"C:\Users\davit.baghdasaryan1\Coding\dit\crates\dit_core\src\tree.rs")?;

    dit.commit("Davit Baghdasaryan", "Commit No3")?;

    Ok(())
}

