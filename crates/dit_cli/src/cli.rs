use std::path::{PathBuf};
use clap::{Parser, Subcommand, ValueEnum};

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
        count: isize
    },

    Status,

    Branch {
        #[command(subcommand)]
        command: BranchCommand,
    },

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
    },

    Reset {
        commit: String,

        #[arg(value_enum, default_value_t = ResetMode::Mixed)]
        mode: ResetMode,
    },

    Clear,
}

#[derive(Subcommand)]
pub enum BranchCommand {
    Switch {
        name: String,

        #[arg(long)]
        hard: bool,
    },

    New {
        name: String,
    },

    Remove {
        name: String,
    }
}

#[derive(Copy, Clone, Debug, ValueEnum)]
pub enum ResetMode {
    Soft,
    Mixed,
    Hard,
}
