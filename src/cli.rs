use chrono::Datelike;
use clap::{Parser, Subcommand};
use configparser::ini::Ini;
use std::env;
use std::error::Error;

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
        #[arg(short, long, default_value_t = get_user().unwrap(), conflicts_with = "template")]
        user: String,
        #[arg(short, long, default_value_t = chrono::Utc::now().year() as u16, conflicts_with = "template")]
        year: u16,
        #[arg(short, long)]
        template: bool,
    },
    Add {
        license: String,
        #[arg(short, long, default_value_t = get_user().unwrap())]
        user: String,
        #[arg(short, long, default_value_t = chrono::Utc::now().year() as u16)]
        year: u16,
    },
}

fn get_user() -> Result<String, Box<dyn Error>> {
    let license_author_name = env::var("LICENSE_AUTHOR_NAME");
    if let Ok(name) = license_author_name {
        Ok(name)
    } else {
        let home_var = env::var("HOME");
        if let Ok(home_path) = home_var {
            let gitconfig_pathname = format!("{}/.gitconfig", home_path);
            let mut gitconfig = Ini::new();
            gitconfig.load(gitconfig_pathname)?;
            let name = gitconfig.get("user", "name").unwrap();
            Ok(name)
        } else {
            Ok("user".to_string())
        }
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
