use clap::Parser;
use dit_cli::cli::Cli;
use dit_cli::dit_handler::DitHandler;

fn main() -> std::io::Result<()> {
    let mut dit_handler = DitHandler::new()?;
    let cli = Cli::parse();
    dit_handler.handle(cli.command)?;
    Ok(())
}
