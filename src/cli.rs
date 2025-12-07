use clap::{CommandFactory, Parser, Subcommand};
use clap_complete::{Shell, generate};
use std::io;

use crate::commands::{ShellCmd, ToggleCmd};

#[derive(Parser, Debug)]
#[command(
    name = "ferret",
    about = "Control Tool",
    version,
    propagate_version = true
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Command>,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    Shell(ShellCmd),

    Toggle(ToggleCmd),

    Completions {
        #[arg(value_enum)]
        shell: Shell,
    },
}

pub fn print_help() {
    let mut cmd = Cli::command();
    cmd.print_help().unwrap();
    println!();
}

pub fn generate_completions(shell: Shell) {
    let mut cmd = Cli::command();
    let bin_name = cmd.get_name().to_string();
    generate(shell, &mut cmd, bin_name, &mut io::stdout());
}
