mod cli;
mod commands;
mod config;
mod utils;

use std::process::ExitCode;

fn main() -> ExitCode {
    let cli = cli::parse();
    utils::init(cli.theme, cli.no_color);

    let Some(command) = cli.command else {
        cli::print_help();
        return ExitCode::SUCCESS;
    };

    let config = config::Config::load();
    match commands::run(command, &config) {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => {
            utils::error(&err);
            ExitCode::FAILURE
        }
    }
}
