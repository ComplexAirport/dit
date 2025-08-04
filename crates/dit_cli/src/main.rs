use clap::Parser;
use dit_cli::{failure, Cli};
use std::process::ExitCode;


fn main() -> ExitCode {
    let cli = Cli::parse();
    let command = cli.command;
    let result = command.handle();

    match result {
        Ok(_) => ExitCode::SUCCESS,

        Err(e) => {
            failure!("{}", e.to_string());
            ExitCode::FAILURE
        }
    }
}
