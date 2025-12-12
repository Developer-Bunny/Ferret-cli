use std::error::Error;

pub mod shell;
pub mod toggle;

pub use shell::ShellCmd;
pub use toggle::ToggleCmd;

pub trait Runnable<Ctx> {
    fn run(&self, context: Ctx) -> Result<(), Box<dyn Error>>;
}
