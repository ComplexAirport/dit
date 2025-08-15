use crate::subcommands::HandleSubcommand;
use crate::error::CliResult;
use crate::success;
use clap::Args;
use std::path::PathBuf;
use dit_core::errors::DitResult;
use dit_core::helpers::resolve_absolute_path;

#[derive(Args)]
pub struct UnstageSubcommand {
    files: Vec<PathBuf>,
}


impl HandleSubcommand for UnstageSubcommand {
    fn handle(&self) -> CliResult<()> {
        let mut dit = Self::require_dit()?;

        let paths = self.files
            .iter()
            .map(|p| resolve_absolute_path(p))
            .collect::<DitResult<Vec<_>>>()?;

        dit.unstage_files(paths)?;

        success!("Unstaged the files successfully");

        Ok(())
    }
}
