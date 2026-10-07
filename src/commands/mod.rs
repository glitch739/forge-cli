//! Command implementations. Each subcommand lives in its own module.

mod doctor;
mod status;

use crate::cli::Command;
use crate::config::Config;

pub type Result = std::result::Result<(), String>;

pub fn run(command: Command, config: &Config) -> Result {
    match command {
        Command::Status => status::run(config),
        Command::Doctor => doctor::run(config),
    }
}
