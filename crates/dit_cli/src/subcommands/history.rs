use crate::subcommands::HandleSubcommand;
use crate::error::CliResult;
use clap::Args;

#[derive(Args)]
pub struct HistorySubcommand {
    #[arg(
        short, long,
        default_value = "5",
        help = "Number of history entries to show. -1 for all entries.")]
    count: isize,
}


impl HandleSubcommand for HistorySubcommand {
    fn handle(&self) -> CliResult<()> {
        let mut dit = Self::require_dit()?;

        let branch_name = dit.branch();
        let commits = dit.history(self.count)?;

        if let Some(branch_name) = branch_name {
            println!("History for the branch '{branch_name}':\n");
        } else {
            println!("History (detached head):\n");
        }

        for (idx, commit) in commits.iter().enumerate() {
            let hash_slice = &commit.hash; // &commit.hash[0..8];
            println!("{}. {hash_slice}", idx + 1);
            println!("    {} - {}", commit.author, commit.message);
        }

        Ok(())
    }
}
