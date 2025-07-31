use crate::subcommands::HandleSubcommand;
use crate::error::CliResult;
use clap::{Args, ValueEnum};

#[derive(Copy, Clone, Debug, ValueEnum)]
pub enum ResetMode {
    Soft,
    Mixed,
    Hard,
}

#[derive(Args)]
pub struct ResetSubcommand {
    commit: String,

    #[arg(value_enum, default_value_t = ResetMode::Mixed)]
    mode: ResetMode,
}


impl HandleSubcommand for ResetSubcommand {
    fn handle(&self) -> CliResult<()> {
        match self.mode {
            ResetMode::Soft => self.handle_soft(),
            ResetMode::Mixed => self.handle_mixed(),
            ResetMode::Hard => self.handle_hard(),
        }
    }
}

impl ResetSubcommand {
    fn handle_soft(&self) -> CliResult<()> {
        let mut dit = Self::require_dit()?;
        dit.mixed_reset(&self.commit)?;
        println!("[+] Mixed reset to commit '{}'", &self.commit);
        Ok(())
    }

    fn handle_mixed(&self) -> CliResult<()> {
        let mut dit = Self::require_dit()?;
        dit.mixed_reset(&self.commit)?;
        println!("[+] Mixed reset to commit '{}'", &self.commit);
        Ok(())
    }

    fn handle_hard(&self) -> CliResult<()> {
        let mut dit = Self::require_dit()?;
        dit.hard_reset(&self.commit)?;
        println!("[+] Hard reset to commit '{}'", &self.commit);
        Ok(())
    }
}
