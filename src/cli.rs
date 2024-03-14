use chrono::Datelike;
use clap::{Parser, Subcommand};
use configparser::ini::Ini;
use std::env;

pub const LICENSE_ARG: &str = "LICENSE";
pub const LICENSE_AUTHOR_ENV_VARIABLE_NAME: &str = "LICENSE_AUTHOR";

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
        #[arg(short = 't', long = "template")]
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
    let license_author_env_variable_result = env::var(LICENSE_AUTHOR_ENV_VARIABLE_NAME);
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
    use serial_test::serial;
    use std::env;
    use std::fs::File;
    use std::io::Write;

    #[test]
    fn verify_cli() {
        use clap::CommandFactory;
        Cli::command().debug_assert()
    }

    #[test]
    #[serial]
    fn determine_license_author_from_env_variable() {
        let env_variable_value = "license_author_env_variable_value";
        env::set_var(LICENSE_AUTHOR_ENV_VARIABLE_NAME, env_variable_value);
        assert_eq!(determine_license_author(), env_variable_value);
    }

    #[test]
    #[serial]
    fn determine_license_author_from_git_config_file() {
        env::remove_var(LICENSE_AUTHOR_ENV_VARIABLE_NAME);
        let git_config_user_name_value = "git_config_user_name_value";
        let temp_dir = tempfile::tempdir().unwrap();
        env::set_var("HOME", temp_dir.path().to_str().unwrap());
        let git_config_file_pathname = temp_dir.path().join(".gitconfig");
        let mut file = File::create(&git_config_file_pathname).unwrap();
        writeln!(file, "[user]\n\tname = {}", git_config_user_name_value).unwrap();
        file.sync_all().unwrap();
        assert_eq!(determine_license_author(), git_config_user_name_value);
    }

    #[test]
    #[serial]
    fn determine_license_author_from_current_effective_user() {
        env::remove_var(LICENSE_AUTHOR_ENV_VARIABLE_NAME);
        env::remove_var("HOME");
        assert_eq!(determine_license_author(), whoami::username());
    }
}
