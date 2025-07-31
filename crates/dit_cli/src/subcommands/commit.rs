use crate::subcommands::HandleSubcommand;
use crate::error::CliResult;
use clap::Args;

#[derive(Args)]
pub struct CommitSubcommand {
    #[arg(short, long)]
    message: String,

    #[arg(short, long)]
    author: String
}


impl HandleSubcommand for CommitSubcommand {
    fn handle(&self) -> CliResult<()> {
        let mut dit = Self::require_dit()?;
        dit.commit(&self.author, &self.message)?;
        println!("[+] Committed the changes");
        Ok(())
    }
}
