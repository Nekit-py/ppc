mod cli;
use clap::Parser;
use cli::Cli;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let mut cli = Cli::parse();
    cli.create()?;
    Ok(())
}
