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
    /// Print the content of the selected open source licenses
    Show {
        license: String,
        #[arg(short, long, conflicts_with = "template")]
        user: Option<String>,
        #[arg(short, long, conflicts_with = "template")]
        year: Option<u32>,
        #[arg(short, long)]
        template: bool,
    },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verify_cli() {
        use clap::CommandFactory;
        Cli::command().debug_assert()
    }
}
