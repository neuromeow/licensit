use chrono::Datelike;
use clap::{Parser, Subcommand};
use configparser::ini::Ini;
use std::env;

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
    let license_author_name_result = env::var("LICENSE_AUTHOR_NAME");
    if let Ok(license_author_name) = license_author_name_result {
        license_author_name
    } else {
        let home_result = env::var("HOME");
        if let Ok(home) = home_result {
            let git_config_file_pathname = format!("{}/.gitconfig", home);
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
