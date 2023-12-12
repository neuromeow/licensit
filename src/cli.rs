use chrono::Datelike;
use clap::{Parser, Subcommand};
use configparser::ini::Ini;
use std::env;

pub const LICENSE_ARG: &str = "LICENSE";

/// Command line tool to create LICENSE files
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
        #[arg(value_name = LICENSE_ARG)]
        name: String,
        /// The user or organization who holds the license
        #[arg(short = 'u', long = "user", value_name = "USER", default_value_t = determine_license_author(), conflicts_with = "is_template")]
        author: String,
        /// The year the license is in effect
        #[arg(short, long, default_value_t = chrono::Utc::now().year() as u32, conflicts_with = "is_template")]
        year: u32,
        /// License template only, no fillers for user or organization and year
        #[arg(short = 't', long = "template", value_name = "TEMPLATE")]
        is_template: bool,
    },
    /// Add the selected license to the current directory
    Add {
        /// Selected license
        #[arg(value_name = LICENSE_ARG)]
        name: String,
        /// The user or organization who holds the license
        #[arg(short = 'u', long = "user", value_name = "USER", default_value_t = determine_license_author())]
        author: String,
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
                let git_config_user_name_option = git_config.get("user", "name");
                if let Some(name) = git_config_user_name_option {
                    return name;
                }
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
