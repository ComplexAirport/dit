use std::path::{Path, PathBuf};
use clap::Parser;
use dit_cli::cli::Cli;
use dit_cli::dit_handler::DitHandler;
use dit_core::DIT_ROOT;

fn main() -> std::io::Result<()> {
    // firstly, we need to find the .dit folder
    let cwd = std::env::current_dir().expect("failed to get current directory");

    let project_root = find_dit_root(cwd);
    match project_root {
        Some(path) => {
            let mut dit_handler = DitHandler::from(path)?;
            let cli = Cli::parse();

            dit_handler.handle(cli.command)?;
        },

        None => {
            eprintln!("error: not a dit project (or any of the parent directories)");
            eprintln!("hint: initialize with `dit init`");
            std::process::exit(1);
        }
    }

    Ok(())
}


/// Recursively searches for `.dit` starting from `start_dir` \
/// Returns the path to the root of the dit repo if found, None otherwise
fn find_dit_root<P: AsRef<Path>>(start_dir: P) -> Option<PathBuf> {
    let start_dir = start_dir.as_ref();
    let mut current = Some(start_dir);

    while let Some(dir) = current {
        if dir.join(DIT_ROOT).is_dir() {
            return Some(dir.to_path_buf());
        }
        current = dir.parent();
    }
    None
}
