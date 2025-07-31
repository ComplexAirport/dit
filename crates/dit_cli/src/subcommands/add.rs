use crate::subcommands::HandleSubcommand;
use crate::error::CliResult;
use clap::Args;
use dit_core::helpers::resolve_absolute_path;
use std::path::PathBuf;

#[derive(Args)]
pub struct AddSubcommand {
    files: Vec<PathBuf>,
}


impl HandleSubcommand for AddSubcommand {
    fn handle(&self) -> CliResult<()> {
        let mut dit = Self::require_dit()?;

        for file in &self.files {
            let abs_path = resolve_absolute_path(file)?;
            dit.stage(&abs_path)?;
            println!("[+] Added '{}' to the staged files", file.display());
        }
        Ok(())
    }
}
