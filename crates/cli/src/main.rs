use dit_core::dit::Dit;

fn main() -> std::io::Result<()> {
    let mut dit = Dit::from_project(r"C:\Users\davit.baghdasaryan1\Coding\dit")?;

    dit.stage_file(r"C:\Users\davit.baghdasaryan1\Coding\dit\crates\dit_core\src\trees.rs")?;
    dit.stage_file(r"C:\Users\davit.baghdasaryan1\Coding\dit\crates\dit_core\src\commits.rs")?;

    dit.commit("Davit Baghdasaryan", "initial commit")?;

    dit.stage_file(r"C:\Users\davit.baghdasaryan1\Coding\dit\crates\dit_core\src\trees.rs")?;
    dit.stage_file(r"C:\Users\davit.baghdasaryan1\Coding\dit\crates\dit_core\src\commits.rs")?;

    dit.commit("Davit Baghdasaryan", "initial commit")?;

    Ok(())
}

