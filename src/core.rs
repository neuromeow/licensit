use clap::Parser;
use colored::Colorize;
use include_dir::{include_dir, Dir};
use serde::Deserialize;
use std::error::Error;
use std::fs::File;
use std::io::Write;

use crate::cli::{Cli, Commands, LICENSE_ARG};

static LICENSES_DATA_DIR: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/data/licenses");
const LICENSES_DESCRIPTIONS_FILE_BASENAME: &str = "licenses.yml";

#[derive(Debug, Deserialize)]
struct Placeholders {
    author: String,
    year: String,
}

impl Placeholders {
    fn author(&self) -> &str {
        &self.author
    }

    fn year(&self) -> &str {
        &self.year
    }
}

#[derive(Debug, Deserialize)]
struct License {
    name: String,
    full_name: String,
    template: String,
    placeholders: Option<Placeholders>,
}

impl License {
    fn name(&self) -> &str {
        &self.name
    }

    fn full_name(&self) -> &str {
        &self.full_name
    }

    fn template(&self) -> &str {
        &self.template
    }

    fn placeholders(&self) -> &Option<Placeholders> {
        &self.placeholders
    }

    fn fetch_template(&self) -> &str {
        let template_relative_path = self.template();
        let template_file = LICENSES_DATA_DIR.get_file(template_relative_path).unwrap();
        template_file.contents_utf8().unwrap()
    }

    fn render_licence(&self, author: &str, year: &u32) -> String {
        let template = self.fetch_template();
        let placeholders_option = self.placeholders();
        if let Some(placeholders) = placeholders_option {
            let author_placeholder = placeholders.author();
            let year_placeholder = placeholders.year();
            let rendered_license = template.replace(author_placeholder, author);
            return rendered_license.replace(year_placeholder, &year.to_string());
        }
        template.to_string()
    }
}

#[derive(Deserialize)]
struct Licenses {
    licenses: Vec<License>,
}

impl Licenses {
    fn from_description_file() -> Self {
        let description_file = LICENSES_DATA_DIR
            .get_file(LICENSES_DESCRIPTIONS_FILE_BASENAME)
            .unwrap();
        let description_file_content = description_file.contents_utf8().unwrap();
        serde_yaml::from_str::<Licenses>(description_file_content).unwrap()
    }

    fn licenses(&self) -> &Vec<License> {
        &self.licenses
    }

    fn find_license(&self, name: &str) -> Option<&License> {
        self.licenses()
            .iter()
            .find(|&license| license.name() == name)
    }

    fn fetch_licenses_names(&self) -> Vec<String> {
        self.licenses()
            .iter()
            .map(|license| license.name().to_string())
            .collect()
    }

    fn fetch_licenses_full_names(&self) -> Vec<String> {
        self.licenses()
            .iter()
            .map(|license| license.full_name().to_string())
            .collect()
    }

    fn fetch_formatted_licenses_names_and_full_names(&self) -> Vec<String> {
        self.fetch_licenses_names()
            .iter()
            .zip(&self.fetch_licenses_full_names())
            .map(|(name, full_name)| format!("{: <16}{}", name, full_name))
            .collect()
    }
}

fn render_invalid_value_error_message(invalid_arg: &str, possible_values: &[String]) -> String {
    let formatted_possible_values = possible_values
        .iter()
        .map(|value| value.green().to_string())
        .collect::<Vec<String>>()
        .join(", ");
    format!(
        "{}: invalid value for '{}'\n\nPossible values: {}\n\nFor more information, try '{}'.",
        "error".red(),
        format!("<{}>", invalid_arg).bold(),
        formatted_possible_values,
        "--help".bold()
    )
}

fn render_nonexistent_license_error(licenses: &Licenses) -> String {
    let licenses_names = licenses.fetch_licenses_names();
    render_invalid_value_error_message(LICENSE_ARG, &licenses_names)
}

pub fn run() -> Result<(), Box<dyn Error>> {
    let licenses = Licenses::from_description_file();
    let cli = Cli::parse();
    match &cli.command {
        Commands::List => {
            let formatted_licenses_names_and_full_names =
                licenses.fetch_formatted_licenses_names_and_full_names();
            for formatted_license_name_and_full_name in formatted_licenses_names_and_full_names {
                println!("{}", formatted_license_name_and_full_name);
            }
        }
        Commands::Show {
            name,
            author,
            year,
            is_template,
        } => {
            let license_option = licenses.find_license(name);
            if let Some(license) = license_option {
                if *is_template {
                    let template = license.fetch_template();
                    print!("{}", template);
                } else {
                    let rendered_license = license.render_licence(author, year);
                    print!("{}", rendered_license);
                }
            } else {
                // This error scenario can also be handled using `clap`.
                let nonexistent_license_error = render_nonexistent_license_error(&licenses);
                eprintln!("{}", nonexistent_license_error);
                std::process::exit(2);
            }
        }
        Commands::Add { name, author, year } => {
            let license_option = licenses.find_license(name);
            if let Some(license) = license_option {
                let rendered_license = license.render_licence(author, year);
                let mut rendered_license_file = File::create("LICENSE")?;
                rendered_license_file.write_all(rendered_license.as_bytes())?;
            } else {
                // This error scenario can also be handled using `clap`.
                let nonexistent_license_error = render_nonexistent_license_error(&licenses);
                eprintln!("{}", nonexistent_license_error);
                std::process::exit(2);
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn render_invalid_value_error_message_with_single_possible_value() {
        let invalid_value = "invalid_value";
        let possible_values = vec!["mit".to_string()];
        let expected_output = format!(
            "{}: invalid value for '{}'\n\nPossible values: {}\n\nFor more information, try '{}'.",
            "error".red(),
            format!("<{}>", invalid_value).bold(),
            possible_values[0].green(),
            "--help".bold()
        );
        assert_eq!(
            render_invalid_value_error_message(invalid_value, &possible_values),
            expected_output
        );
    }

    #[test]
    fn render_invalid_value_error_message_with_multiple_possible_values() {
        let invalid_value = "invalid_value";
        let possible_values = vec![
            "agpl-3.0".to_string(),
            "mit".to_string(),
            "green".to_string(),
        ];
        let expected_output = format!(
            "{}: invalid value for '{}'\n\nPossible values: {}\n\nFor more information, try '{}'.",
            "error".red(),
            format!("<{}>", invalid_value).bold(),
            possible_values
                .iter()
                .map(|value| value.green().to_string())
                .collect::<Vec<String>>()
                .join(", "),
            "--help".bold()
        );
        assert_eq!(
            render_invalid_value_error_message(invalid_value, &possible_values),
            expected_output
        );
    }
}
