mod cli;
mod commands;

use clap::Parser;
use std::error::Error;

use cli::{Cli, Command};
use commands::Runnable;

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();

    match cli.command {
        Some(Command::Shell(cmd)) => cmd.run(),
        Some(Command::Toggle(cmd)) => cmd.run(),
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
