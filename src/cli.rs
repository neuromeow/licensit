use chrono::Datelike;
use clap::{Parser, Subcommand};
use configparser::ini::Ini;
use std::env;

use crate::util::LICENSES_ABBREVIATIONS;

/// Console application for working with open source licenses
#[derive(Parser)]
#[command(version)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Print a list of all available licenses
    List,
    /// Print the content of the selected license
    Show {
        /// Selected license
        #[arg(value_parser = LICENSES_ABBREVIATIONS)]
        license: String,
        /// The user or organization who holds the license
        #[arg(short, long, default_value_t = determine_license_author(), conflicts_with = "template")]
        user: String,
        /// The year the license is in effect
        #[arg(short, long, default_value_t = chrono::Utc::now().year() as u32, conflicts_with = "template")]
        year: u32,
        /// License template only, no fillers for user or organization and year
        #[arg(short, long)]
        template: bool,
    },
    /// Add the selected license to the current directory
    Add {
        /// Selected license
        #[arg(value_parser = LICENSES_ABBREVIATIONS)]
        license: String,
        /// The user or organization who holds the license
        #[arg(short, long, default_value_t = determine_license_author())]
        user: String,
        /// The year the license is in effect
        #[arg(short, long, default_value_t = chrono::Utc::now().year() as u32)]
        year: u32,
    },
}

fn determine_license_author() -> String {
    let license_author_env_variable_result = env::var("LICENSE_AUTHOR");
    if let Ok(license_author_env_variable) = license_author_env_variable_result {
        license_author_env_variable
    } else {
        let home_env_variable_result = env::var("HOME");
        if let Ok(home_env_variable) = home_env_variable_result {
            let git_config_file_pathname = format!("{}/.gitconfig", home_env_variable);
            let mut git_config = Ini::new();
            if git_config.load(git_config_file_pathname).is_ok() {
                return git_config.get("user", "name").unwrap();
            }
        }
        whoami::username()
    }
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
