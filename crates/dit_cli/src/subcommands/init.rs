use crate::subcommands::HandleSubcommand;
use crate::error::{CliResult, DitCliError};
use clap::Args;
use dit_core::{Dit, DIT_ROOT};
use crate::success;

#[derive(Args)]
pub struct InitSubcommand;


impl HandleSubcommand for InitSubcommand {
    fn handle(&self) -> CliResult<()> {
        let cwd = std::env::current_dir()
            .map_err(|_| DitCliError::CwdError)?;

        // check if the dit is already initialized or existed before
        let is_new = !cwd.join(DIT_ROOT).is_dir();

        let mut dit = Dit::from(&cwd)?;

        // default behavior:
        // if no head branch is found, a default "main" branch will be created
        if dit.get_branch().is_none() {
            dit.create_branch("main")?;
        }

        if is_new {
            success!("Initialized a new dit directory in '{}'.", cwd.display());
        } else {
            success!("Reinitialized the existing dit directory in '{}'.", cwd.display());
        }

        Ok(())
    }
}
