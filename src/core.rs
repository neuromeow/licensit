use clap::Parser;
use colored::*;
use include_dir::{include_dir, Dir};
use serde::Deserialize;
use std::error::Error;
use std::fs::File;
use std::io::Write;

use crate::cli::{Cli, Commands};

static LICENSES_DIR: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/data");
const LICENSES_DESCRIPTIONS_FILE_BASENAME: &str = "licenses.yml";

#[derive(Debug, Deserialize)]
struct Placeholders {
    author: String,
    year: String,
}

impl Placeholders {
    fn get_author(&self) -> &str {
        &self.author
    }

    fn get_year(&self) -> &str {
        &self.year
    }
}

#[derive(Debug, Deserialize)]
struct License {
    name: String,
    full_name: String,
    template_path: String,
    placeholders: Option<Placeholders>,
}

impl License {
    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_full_name(&self) -> &str {
        &self.full_name
    }

    fn get_template_path(&self) -> &str {
        &self.template_path
    }

    fn get_placeholders(&self) -> &Option<Placeholders> {
        &self.placeholders
    }

    fn format_license_names(&self) -> String {
        format!("{: <12}{}", self.get_name(), self.get_full_name())
    }

    fn fetch_template(&self) -> &str {
        let template_relative_path = self.get_template_path();
        let template_file = LICENSES_DIR.get_file(template_relative_path).unwrap();
        template_file.contents_utf8().unwrap()
    }

    fn render_licence(&self, author: &str, year: &u32) -> String {
        let template = self.fetch_template();
        let placeholders_option = self.get_placeholders();
        if let Some(placeholders) = placeholders_option {
            let author_placeholder = placeholders.get_author();
            let year_placeholder = placeholders.get_year();
            let rendered_license = template.replace(author_placeholder, author);
            return rendered_license.replace(year_placeholder, &year.to_string());
        }
        template.to_string()
    }
}

#[derive(Debug, Deserialize)]
struct Licenses {
    licenses: Vec<License>,
}

impl Licenses {
    fn from_description_file() -> Self {
        let description_file = LICENSES_DIR
            .get_file(LICENSES_DESCRIPTIONS_FILE_BASENAME)
            .unwrap();
        let description_file_content = description_file.contents_utf8().unwrap();
        serde_yaml::from_str::<Licenses>(description_file_content).unwrap()
    }

    fn get_licenses(&self) -> &Vec<License> {
        &self.licenses
    }

    fn find_license(&self, name: &str) -> Option<&License> {
        self.get_licenses()
            .iter()
            .find(|&license| license.get_name() == name)
    }

    fn fetch_licenses_names(&self) -> Vec<String> {
        self.get_licenses()
            .iter()
            .map(|license| license.get_name().to_string())
            .collect()
    }

    fn fetch_formatted_licenses_names(&self) -> Vec<String> {
        self.get_licenses()
            .iter()
            .map(|license| license.format_license_names())
            .collect()
    }
}

pub fn run() -> Result<(), Box<dyn Error>> {
    let licenses = Licenses::from_description_file();
    let cli = Cli::parse();
    match &cli.command {
        Commands::List => {
            let licenses_names = licenses.fetch_formatted_licenses_names();
            for license_names in licenses_names {
                println!("{}", license_names);
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
                    println!("{}", template);
                } else {
                    let rendered_license = license.render_licence(author, year);
                    println!("{}", rendered_license);
                }
            } else {
                eprintln!("error: invalid value for '<LICENSE>'. Possible values: {}\n\nFor more information, try '--help'.",
                          licenses.fetch_licenses_names()
                              .iter()
                              .map(|name| name.green().to_string())
                              .collect::<Vec<String>>()
                              .join(", "));
            }
        }
        Commands::Add {
            name,
            author,
            year,
        } => {
            let license_option = licenses.find_license(name);
            if let Some(license) = license_option {
                let rendered_license = license.render_licence(author, year);
                let mut rendered_license_file = File::create("LICENSE")?;
                rendered_license_file.write_all(rendered_license.as_bytes())?;
            } else {
                eprintln!("error: invalid value for '<LICENSE>'. Possible values: {}\n\nFor more information, try '--help'.",
                          licenses.fetch_licenses_names()
                              .iter()
                              .map(|name| name.green().to_string())
                              .collect::<Vec<String>>()
                              .join(", "));
            }
        }
    }
    Ok(())
}
