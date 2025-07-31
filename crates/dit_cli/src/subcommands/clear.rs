use crate::subcommands::HandleSubcommand;
use crate::error::CliResult;
use clap::Args;

#[derive(Args)]
pub struct ClearSubcommand;

impl HandleSubcommand for ClearSubcommand {
    fn handle(&self) -> CliResult<()> {
        let mut dit = Self::require_dit()?;
        dit.clear_stage()?;
        println!("[+] Cleared the stage.");
        Ok(())
    }
}
