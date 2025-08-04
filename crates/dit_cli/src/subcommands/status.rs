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
        // todo: this needs to be changed
        // also need to display files which are not tracked
        // and also the files which were changed compared to the last commit
        // or last time being staged

        let dit = Self::require_dit()?;
        let branch_name = dit.branch();
        let head_commit = dit.head_commit();

        match branch_name {
            Some(b) => println!("On branch '{}'", style(b).green().bold()),
            None => println!("No current branch")
        }

        match head_commit {
            Some(h) => println!("Parent commit: '{}'", style(h).yellow().bold()),
            None => println!("No commits yet")
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
