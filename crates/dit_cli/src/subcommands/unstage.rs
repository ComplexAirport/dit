use crate::subcommands::HandleSubcommand;
use crate::error::CliResult;
use clap::Args;
use dit_core::helpers::resolve_absolute_path;
use std::path::PathBuf;

#[derive(Args)]
pub struct UnstageSubcommand {
    files: Vec<PathBuf>,
}


impl HandleSubcommand for UnstageSubcommand {
    fn handle(&self) -> CliResult<()> {
        let mut dit = Self::require_dit()?;
        for file in &self.files {
            let abs_path = resolve_absolute_path(file)?;
            dit.unstage(&abs_path)?;
            println!("[+] Unstaged the file `{}`", file.display());
        }
        Ok(())
    }
}
