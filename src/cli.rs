use clap::builder::styling::{AnsiColor, Effects, Style, Styles};
use clap::{ColorChoice, CommandFactory, FromArgMatches, Parser, Subcommand};

use crate::utils::ThemeName;

const LOGO: &str = "\
███████╗ ██████╗ ██████╗  ██████╗ ███████╗
██╔════╝██╔═══██╗██╔══██╗██╔════╝ ██╔════╝
█████╗  ██║   ██║██████╔╝██║  ███╗█████╗
██╔══╝  ██║   ██║██╔══██╗██║   ██║██╔══╝
██║     ╚██████╔╝██║  ██║╚██████╔╝███████╗
╚═╝      ╚═════╝ ╚═╝  ╚═╝ ╚═════╝ ╚══════╝";

const ACCENT: Style = AnsiColor::Cyan.on_default().effects(Effects::BOLD);
const LITERAL: Style = AnsiColor::Cyan.on_default();
const MUTED: Style = AnsiColor::BrightBlack.on_default();

const STYLES: Styles = Styles::styled()
    .header(ACCENT)
    .usage(ACCENT)
    .literal(LITERAL)
    .placeholder(MUTED);

/// Developer and operations toolkit for your machine and your servers.
#[derive(Debug, Parser)]
#[command(name = "forge", version, propagate_version = true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Command>,

    /// Color theme
    #[arg(
        long,
        global = true,
        value_enum,
        default_value_t,
        value_name = "NAME",
        hide_default_value = true
    )]
    pub theme: ThemeName,

    /// Disable colored output
    #[arg(long, global = true)]
    pub no_color: bool,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    /// Show the current state of the environment
    Status,
    /// Check that the environment is set up correctly
    Doctor,
}

pub fn parse() -> Cli {
    let matches = command().get_matches();
    Cli::from_arg_matches(&matches).unwrap_or_else(|err| err.exit())
}

pub fn print_help() {
    let _ = command().print_help();
}

fn command() -> clap::Command {
    // Help is rendered while parsing, before flags are available,
    // so `--no-color` has to be honored up front.
    let color = if std::env::args_os().any(|arg| arg == "--no-color") {
        ColorChoice::Never
    } else {
        ColorChoice::Auto
    };

    Cli::command()
        .color(color)
        .styles(STYLES)
        .before_help(banner())
        .after_help(examples())
}

fn banner() -> String {
    let version = env!("CARGO_PKG_VERSION");
    format!("{ACCENT}{LOGO}{ACCENT:#}  {MUTED}v{version}{MUTED:#}")
}

fn examples() -> String {
    let rows = [
        ("forge doctor", "Check that this machine is ready"),
        ("forge status", "Show the current environment"),
        ("forge --theme github doctor", "Use a different color theme"),
    ];
    let mut text = format!("{ACCENT}Examples:{ACCENT:#}");
    for (cmd, about) in rows {
        text.push_str(&format!("\n  {LITERAL}{cmd:<30}{LITERAL:#}{about}"));
    }
    text
}
