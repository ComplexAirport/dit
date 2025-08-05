use crate::subcommands::HandleSubcommand;
use crate::error::CliResult;
use clap::Args;
use console::style;

#[derive(Args)]
pub struct StatusSubcommand {
    #[arg(
        short, long,
        default_value = "5",
        help = "Number of history entries to show. -1 for all entries.")]
    count: isize,
}


impl HandleSubcommand for StatusSubcommand {
    fn handle(&self) -> CliResult<()> {
        let dit = Self::require_dit()?;
        let branch_name = dit.get_branch();
        let head_commit = dit.get_head_commit();

        match branch_name {
            Some(b) => println!("On branch '{}'", style(b).green().bold()),
            None => println!("No current branch")
        }

        match head_commit {
            Some(h) => println!("Parent commit: '{}'", style(h).yellow().bold()),
            None => println!("No commits yet")
        }

        let status =  dit.get_status()?;

        let unchanged = status.staged_files();
        if !unchanged.is_empty() {
            println!("\nFiles to be commited:");
            for path in unchanged {
                println!("\t{}", style(path.to_string_lossy().to_string()).green().bold());
            }
        }

        let modified = status.modified_files();
        if !modified.is_empty() {
            println!("\nModified files:");
            for path in modified {
                println!("\t{}", style(path.to_string_lossy().to_string()).yellow().bold());
            }
        }

        let deleted = status.deleted_files();
        if !deleted.is_empty() {
            println!("\nDeleted files:");
            for path in deleted {
                println!("\t{}", style(path.to_string_lossy().to_string()).red().bold());
            }
        }

        let untracked = status.untracked_files();
        if !untracked.is_empty() {
            // todo: compare with parent tree
            println!("\nUntracked files:");
            for path in untracked {
                println!("\t{}", style(path.to_string_lossy().to_string()).dim().bold());
            }
        }

        Ok(())
    }
}
