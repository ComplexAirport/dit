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

        let staged_files = dit.get_stage().files;
        if staged_files.is_empty() {
            println!("No staged files");
        } else {
            println!("Staged files:");
            for path in staged_files.keys() {
                println!("    {}", style(path.display()).bright());
            }
        }

        Ok(())
    }
}
