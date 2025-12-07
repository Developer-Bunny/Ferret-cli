use std::error::Error;

pub mod shell;
pub mod toggle;

pub use shell::ShellCmd;
pub use toggle::ToggleCmd;

pub trait Runnable {
    fn run(self) -> Result<(), Box<dyn Error>>;
}
