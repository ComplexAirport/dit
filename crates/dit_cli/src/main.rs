use clap::Parser;
use dit_cli::{
    cli::Cli,
    dit_handler::DitHandler
};
use std::process::ExitCode;

fn main() -> ExitCode {
    let dit_handler = DitHandler::new();

    match dit_handler {
        Ok(mut dit_handler) => {
            let cli = Cli::parse();
            let result = dit_handler.handle(cli.command);
            match result {
                Ok(_) => ExitCode::SUCCESS,
                Err(e) => {
                    eprintln!("{e}");
                    ExitCode::FAILURE
                }
            }
        }
        Err(e) => {
            eprintln!("{e}");
            ExitCode::FAILURE
        }
    }
}
