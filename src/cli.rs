use clap::{Parser, Subcommand};

/// Forge — a cross-platform developer and operations toolkit.
#[derive(Debug, Parser)]
#[command(name = "forge", version, about, propagate_version = true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    /// Show the current state of the environment
    Status,
    /// Check that the local environment is set up correctly
    Doctor,
}
