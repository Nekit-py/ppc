mod cli;
use clap::Parser;
use cli::Cli;

fn main() {
    let mut cli = Cli::parse();
    cli.create();
}
