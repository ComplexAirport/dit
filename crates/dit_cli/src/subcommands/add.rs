use crate::subcommands::HandleSubcommand;
use crate::error::CliResult;
use crate::success;
use clap::Args;
use dit_core::helpers::path_to_string;
use std::path::PathBuf;

#[derive(Args)]
pub struct AddSubcommand {
    files: Vec<PathBuf>,
}


impl HandleSubcommand for AddSubcommand {
    fn handle(self) -> CliResult<()> {
        let mut dit = Self::require_dit()?;

        let globs = self.files
            .into_iter()
            .map(|p| path_to_string(&p));

        let expanded_globs = dit.expand_globs_cwd(globs)?;

        dit.add_files(expanded_globs)?;

        success!("Staged the files successfully");

        Ok(())
    }
}
