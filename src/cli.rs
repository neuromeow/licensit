use clap::{Parser, Subcommand};

/// Console application for working with open source licenses
#[derive(Parser)]
#[command(version)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Print a list of available open source licenses
    List,
}
