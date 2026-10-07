//! Command implementations. Each subcommand maps to one function here.

use crate::cli::Command;
use crate::config::Config;
use crate::utils;

pub type Result = std::result::Result<(), String>;

pub fn run(command: Command, config: &Config) -> Result {
    match command {
        Command::Status => status(config),
        Command::Doctor => doctor(config),
    }
}

fn status(config: &Config) -> Result {
    utils::header("status");
    utils::field("target", "local");
    utils::field("platform", &utils::platform());
    utils::field("config", &config.display_path());
    println!();
    utils::hint("Status reporting is not implemented yet.");
    Ok(())
}

fn doctor(_config: &Config) -> Result {
    utils::header("doctor");
    utils::check("forge binary", true);
    println!();
    utils::hint("Environment checks are not implemented yet.");
    Ok(())
}
