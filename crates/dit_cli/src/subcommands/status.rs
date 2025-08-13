use crate::subcommands::HandleSubcommand;
use crate::error::CliResult;
use dit_core::api_models::status::ChangeType;
use clap::Args;
use console::style;
use dit_core::helpers::path_to_string;

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
        let branch_name = dit.get_branch()?;
        let head_commit = dit.get_head_commit()?;

        match branch_name {
            Some(b) => println!("On branch '{}'", style(b).green().bold()),
            None => println!("No current branch")
        }

        match head_commit {
            Some(h) => println!("Parent commit: '{}'", style(h).yellow().bold()),
            None => println!("No commits yet")
        }

        let status =  dit.get_status()?;

        if status.has_any_tracked() {
            println!("Changed to be commited:");
            for path in status.get_tracked(ChangeType::New) {
                println!("\tnew file: {}", style(path_to_string(path)).green().bold());
            }
            for path in status.get_tracked(ChangeType::Modified) {
                println!("\tmodified: {}", style(path_to_string(path)).green().bold());
            }
            for path in status.get_tracked(ChangeType::Deleted) {
                println!("\tdeleted: {}", style(path_to_string(path)).red().strikethrough());
            }
        }

        if status.has_any_unstaged() {
            println!("\nUnstaged changes:");
            for path in status.get_unstaged(ChangeType::Modified) {
                println!("\tmodified: {}", style(path_to_string(path)).yellow().bold());
            }
            for path in status.get_unstaged(ChangeType::Deleted) {
                println!("\tdeleted: {}", style(path_to_string(path)).red().strikethrough());
            }
        }

        if status.has_any_untracked() {
            println!("\nUntracked files:");
            for path in status.get_untracked() {
                println!("\t{}", style(path_to_string(path)).dim().bold());
            }
        }

        Ok(())
    }
}
