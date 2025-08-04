use crate::subcommands::HandleSubcommand;
use crate::error::CliResult;
use clap::{Args, Subcommand};

#[derive(Subcommand)]
pub enum BranchCommand {
    New {
        name: String,
    },

    Switch {
        name: String,

        #[arg(long)]
        hard: bool,
    },

    Remove {
        name: String,
    },

    Merge {
        from: String
    }
}



#[derive(Args)]
pub struct BranchSubcommand {
    #[command(subcommand)]
    command: BranchCommand,
}


impl HandleSubcommand for BranchSubcommand {
    fn handle(&self) -> CliResult<()> {
        match &self.command {
            BranchCommand::New { name } => self.handle_new(name),
            BranchCommand::Switch { name, hard } => self.handle_switch(name, *hard),
            BranchCommand::Remove { name } => self.handle_remove(name),
            BranchCommand::Merge { from } => self.handle_merge(from),
        }
    }
}


impl BranchSubcommand {
    fn handle_new(&self, name: &String) -> CliResult<()> {
        let mut dit = Self::require_dit()?;
        dit.create_branch(&name)?;
        println!("[+] Created a new branch '{name}'");
        Ok(())
    }

    fn handle_switch(&self, name: &String, hard: bool) -> CliResult<()> {
        let mut dit = Self::require_dit()?;
        dit.switch_branch(&name, hard)?;
        println!("[+] Switched to branch '{name}'");
        Ok(())
    }

    fn handle_remove(&self, name: &String) -> CliResult<()> {
        let mut dit = Self::require_dit()?;
        dit.remove_branch(name)?;
        println!("[+] Removed the branch '{name}'");
        Ok(())
    }

    fn handle_merge(&self, from: &String) -> CliResult<()> {
        let mut dit = Self::require_dit()?;
        dit.merge_branch(from)?;
        println!("[+] Merged the branch '{from}' into the current branch");
        Ok(())
    }
}

