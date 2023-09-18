use chrono::Datelike;
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
        #[arg(short, long, default_value_t = get_user(), conflicts_with = "template")]
        user: String,
        #[arg(short, long, default_value_t = chrono::Utc::now().year() as u16, conflicts_with = "template")]
        year: u16,
        #[arg(short, long)]
        template: bool,
    },
    Add {
        license: String,
        #[arg(short, long, default_value_t = get_user())]
        user: String,
        #[arg(short, long, default_value_t = chrono::Utc::now().year() as u16)]
        year: u16,
    },
}

fn get_user() -> String {
    "user".to_string()
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
