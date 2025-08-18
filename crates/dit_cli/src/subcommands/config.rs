use crate::error::CliResult;
use crate::subcommands::HandleSubcommand;
use crate::{hint, info, success};
use clap::{Args, Subcommand};
use console::style;

impl HandleSubcommand for ConfigSubcommand {
    fn handle(self) -> CliResult<()> {
        match self.command {
            ConfigCommand::Set { command } => Self::handle_set(command),
            ConfigCommand::Get { command } => Self::handle_get(command),
        }
    }
}

impl ConfigSubcommand {
    fn handle_set(cmd: ConfigSetCommand) -> CliResult<()> {
        let mut dit = Self::require_dit()?;

        match cmd {
            ConfigSetCommand::UserName { value} => {
                let msg = format!("Set user.name to {value}");
                dit.config_set_user_name(value)?;
                success!("{msg}")
            },
            ConfigSetCommand::UserEmail { value } => {
                let msg = format!("Set user.email to {value}");
                dit.config_set_user_email(value)?;
                success!("{msg}")
            }
        }

        Ok(())
    }

    fn handle_get(cmd: ConfigGetCommand) -> CliResult<()> {
        let dit = Self::require_dit()?;

        match cmd {
            ConfigGetCommand::UserName => {
                if let Some(value) = dit.config_get_user_name()? {
                    info!("user.name: '{}'", style(value).green().bold());
                } else {
                    info!("user.name: {}", style("none").yellow().bold());
                    hint!("Set with `dit config set user.name <NAME>`");
                }
            }
            ConfigGetCommand::UserEmail => {
                if let Some(value) = dit.config_get_user_email()? {
                    info!("user.email: '{}'", style(value).green().bold());
                } else {
                    info!("user.email: {}", style("none").yellow().bold());
                    hint!("Set email with `dit config set user.email <EMAIL>`");
                }
            }
            ConfigGetCommand::User => {
                if let Some(value) = dit.config_get_user()? {
                    info!("user: '{}'", style(value).green().bold());
                } else {
                    info!("user: {}", style("none").yellow().bold());
                    hint!("Set name with `dit config set user.name <NAME>`");
                    hint!("Set email with `dit config set user.email <EMAIL>`");
                }
            }
        }

        Ok(())
    }
}


#[derive(Args)]
pub struct ConfigSubcommand {
    #[command(subcommand)]
    command: ConfigCommand,
}

#[derive(Subcommand)]
pub enum ConfigCommand {
    Set {
        #[command(subcommand)]
        command: ConfigSetCommand
    },

    Get {
        #[command(subcommand)]
        command: ConfigGetCommand,
    }
}

#[derive(Subcommand)]
pub enum ConfigSetCommand {
    #[clap(name = "user.name")]
    UserName {
        value: String
    },

    #[clap(name = "user.email")]
    UserEmail {
        value: String
    },
}

#[derive(Subcommand)]
pub enum ConfigGetCommand {
    #[clap(name = "user.name")]
    UserName,

    #[clap(name = "user.email")]
    UserEmail,

    #[clap(name = "user")]
    User,
}