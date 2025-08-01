use crate::subcommands::HandleSubcommand;
use crate::error::CliResult;
use clap::Args;

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
        // todo: this needs to be changed
        // also need to display files which are not tracked
        // and also the files which were changed compared to the last commit
        // or last time being staged

        let dit = Self::require_dit()?;
        let branch_name = dit.branch();
        let head_commit = dit.head_commit();

        if let Some(branch_name) = branch_name {
            println!("On branch '{branch_name}'")
        } else {
            println!("No current branch");
        }

        if let Some(head_commit) = head_commit {
            println!("Parent commit: '{head_commit}'")
        } else {
            println!("No commits yet");
        }

        dit.with_stage(|staged_files| {
            if !staged_files.files.is_empty() {
                println!("Changes to be committed: ");
                for path in staged_files.files.keys() {
                    println!("    {}", path.display());
                }
            } else {
                println!();
            }
        });

        Ok(())
    }
}
