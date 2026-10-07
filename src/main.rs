mod cli;
mod commands;
mod config;
mod utils;

use std::process::ExitCode;

use clap::Parser;

fn main() -> ExitCode {
    let cli = cli::Cli::parse();
    let config = config::Config::load();

    match commands::run(cli.command, &config) {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => {
            utils::error(&err);
            ExitCode::FAILURE
        }
    }
}
