use crate::subcommands::HandleSubcommand;
use crate::error::CliResult;
use crate::success;
use clap::Args;
use dit_core::helpers::resolve_absolute_path;
use std::path::PathBuf;
use dit_core::errors::DitResult;

#[derive(Args)]
pub struct AddSubcommand {
    files: Vec<PathBuf>,
}


impl HandleSubcommand for AddSubcommand {
    fn handle(&self) -> CliResult<()> {
        let mut dit = Self::require_dit()?;

        let paths = self.files
            .iter()
            .map(|p| resolve_absolute_path(p))
            .collect::<DitResult<Vec<_>>>()?;

        dit.add_files(paths)?;

        success!("Staged the files successfully");

        Ok(())
    }
}
