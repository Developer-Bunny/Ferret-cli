use clap::Args;
use std::error::Error;

use super::Runnable;

#[derive(Args, Debug)]
pub struct ToggleCmd {
    pub workspace: String,
}

impl Runnable for ToggleCmd {
    fn run(self) -> Result<(), Box<dyn Error>> {
        println!("Toggling workspace: {}", self.workspace);
        Ok(())
    }
}
