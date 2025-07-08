use std::path::{PathBuf};
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "dit")]
#[command(version = "1.0")]
#[command(about = "Dit - a minimal version control system similar to Git")]
pub struct Cli {
    #[command(subcommand)]
    pub command: CommandKind,
}

#[derive(Subcommand)]
pub enum CommandKind {
    Init,

    History {
        #[arg(short, long, default_value = "5")]
        count: usize
    },

    Status,

    Add {
        files: Vec<PathBuf>,
    },

    Unstage {
        files: Vec<PathBuf>,
    },

    Commit {
        #[arg(short, long)]
        message: String,

        #[arg(short, long)]
        author: String
    }
}
