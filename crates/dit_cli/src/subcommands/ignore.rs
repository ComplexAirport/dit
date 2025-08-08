use clap::{Args, Subcommand};
use crate::error::CliResult;
use crate::subcommands::HandleSubcommand;
use crate::success;

#[derive(Args)]
pub struct IgnoreSubcommand {
    #[command(subcommand)]
    command: IgnoreCommand,
}

#[derive(Subcommand)]
pub enum IgnoreCommand {
    Add {
        pattern: String
    },

    Remove {
        pattern: String
    }
}

impl HandleSubcommand for IgnoreSubcommand {
    fn handle(&self) -> CliResult<()> {
        let dit = Self::require_dit()?;

        match &self.command {
            IgnoreCommand::Add { pattern } => {
                dit.ignore(pattern)?;
                success!("Added the files and directories from pattern '{pattern}' to the ignore list.");
                Ok(())
            }

            IgnoreCommand::Remove { pattern } => {
                dit.unignore(pattern)?;
                success!("Removed the files and directories from pattern '{pattern}' from the ignore list.");
                Ok(())
            }
        }
    }
}

