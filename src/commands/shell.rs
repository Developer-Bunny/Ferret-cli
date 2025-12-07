use clap::Args;
use std::error::Error;

use super::Runnable;

#[derive(Args, Debug)]
pub struct ShellCmd {
    pub message: Vec<String>,

    #[arg(short, long)]
    pub daemon: bool,

    #[arg(short, long)]
    pub show: bool,

    #[arg(short, long)]
    pub log: bool,

    #[arg(short, long)]
    pub kill: bool,

    #[arg(long = "log-rules", value_name = "RULES")]
    pub log_rules: Option<String>,
}

impl Runnable for ShellCmd {
    fn run(self) -> Result<(), Box<dyn Error>> {
        if self.kill {
            println!("Killing shell…");
        } else if self.daemon {
            println!("Starting shell as daemon…");
        }

        if !self.message.is_empty() {
            println!("Sending message: {:?}", self.message);
        }

        Ok(())
    }
}
