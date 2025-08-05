use crate::error::{CliResult, DitCliError};
use std::path::{Path, PathBuf};
use dit_core::{Dit, DIT_ROOT};

pub trait HandleSubcommand {
    fn handle(&self) -> CliResult<()>;

    /// If the dit is initialized in the curren directory, returns a [`Dit`] instance.
    /// Otherwise, prints an error to stderr and exits
    fn require_dit() -> CliResult<Dit> {
        let cwd = std::env::current_dir()
            .map_err(|_| DitCliError::CwdError)?;

        let project_root = find_dit_root(cwd);
        match project_root {
            Some(project_root) => {
                let dit = Dit::from(project_root)?;
                Ok(dit)
            },

            None => {
                eprintln!("error: not a dit project (or any of the parent directories)");
                eprintln!("hint: initialize with `dit init`");
                std::process::exit(1);
            }
        }
    }
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
