mod cli;
mod commands;
mod utils;

use clap::Parser;
use std::error::Error;

use cli::{Cli, Command};
use commands::Runnable;

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();
    let path = utils::paths::Paths::new();

    match cli.command {
        Some(Command::Shell(cmd)) => cmd.run(&path),
        Some(Command::Toggle(cmd)) => cmd.run(&path),
        Some(Command::Completions { shell }) => {
            cli::generate_completions(shell);
            Ok(())
        }
        None => {
            cli::print_help();
            Ok(())
        }
    }
}
