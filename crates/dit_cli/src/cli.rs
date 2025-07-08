use std::path::{PathBuf};
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "dit")]
#[command(version = "1.0")]
#[command(about = "Minimal version control system")]
pub struct Cli {
    #[command(subcommand)]
    pub command: CommandKind,
}

#[derive(Subcommand)]
pub enum CommandKind {
    Add {
        file: PathBuf,
    },

    Unstage {
        file: PathBuf,
    },

    Commit {
        #[arg(short, long)]
        message: String,

        #[arg(short, long)]
        author: String
    }
}
