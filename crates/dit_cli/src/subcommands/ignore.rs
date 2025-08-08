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
        patterns: Vec<String>
    },

    Remove {
        patterns: Vec<String>
    },

    List,
}

impl HandleSubcommand for IgnoreSubcommand {
    fn handle(&self) -> CliResult<()> {
        let dit = Self::require_dit()?;

        match &self.command {
            IgnoreCommand::Add { patterns } => {
                for pattern in patterns {
                    dit.ignore(pattern)?;
                    success!("Added '{pattern}' to the ignored list.");
                }
                Ok(())
            }

            IgnoreCommand::Remove { patterns } => {
                for pattern in patterns {
                    dit.unignore(pattern)?;
                    success!("Removed '{pattern}' from the ignored list.");
                }
                Ok(())
            }

            IgnoreCommand::List => {
                let ignored_list = dit.get_ignored_list()?;
                for pattern in ignored_list.patterns {
                    println!("{pattern}");
                }
                Ok(())
            }
        }
    }
}

