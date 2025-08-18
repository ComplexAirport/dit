use clap::{Parser, Subcommand};
use crate::error::CliResult;
use crate::subcommands::*;

#[derive(Parser)]
#[command(name = "dit")]
#[command(about = "Dit - a minimal version control system similar to Git")]
pub struct Cli {
    #[command(subcommand)]
    pub command: CommandKind,
}

#[derive(Subcommand)]
pub enum CommandKind {
    Init(InitSubcommand),
    History(HistorySubcommand),
    Status(StatusSubcommand),
    Branch(BranchSubcommand),
    Add(AddSubcommand),
    Unstage(UnstageSubcommand),
    Commit(CommitSubcommand),
    Reset(ResetSubcommand),
    Clear(ClearSubcommand),
    Config(ConfigSubcommand),
}

impl CommandKind {
    pub fn handle(self) -> CliResult<()> {
        match self {
            Self::Init(cmd) => cmd.handle(),
            Self::History(cmd) => cmd.handle(),
            Self::Status(cmd) => cmd.handle(),
            Self::Branch(cmd) => cmd.handle(),
            Self::Add(cmd) => cmd.handle(),
            Self::Unstage(cmd) => cmd.handle(),
            Self::Commit(cmd) => cmd.handle(),
            Self::Reset(cmd) => cmd.handle(),
            Self::Clear(cmd) => cmd.handle(),
            Self::Config(cmd) => cmd.handle(),
        }
    }
}
