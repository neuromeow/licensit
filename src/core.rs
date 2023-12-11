use clap::Parser;
use include_dir::{include_dir, Dir};
use serde::Deserialize;
use std::error::Error;
use std::fs::File;
use std::io::Write;

use crate::cli::{Cli, Commands};
use crate::util;

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
    abbreviation: String,
    name: String,
    template_path: String,
    placeholders: Option<Placeholders>,
}

impl License {
    fn get_abbreviation(&self) -> &str {
        &self.abbreviation
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_template_path(&self) -> &str {
        &self.template_path
    }

    fn get_placeholders(&self) -> &Option<Placeholders> {
        &self.placeholders
    }

    fn format_license_names(&self) -> String {
        format!("{: <12}{}", self.get_abbreviation(), self.get_name())
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
    fn from_licenses_file() -> Self {
        let licenses_file = LICENSES_DIR
            .get_file(LICENSES_DESCRIPTIONS_FILE_BASENAME)
            .unwrap();
        let licenses_file_content = licenses_file.contents_utf8().unwrap();
        serde_yaml::from_str::<Licenses>(licenses_file_content).unwrap()
    }

    fn get_licenses(&self) -> &Vec<License> {
        &self.licenses
    }

    fn get_license(&self, abbreviation: &str) -> Option<&License> {
        self.get_licenses()
            .iter()
            .find(|&license| license.get_abbreviation() == abbreviation)
    }

    fn fetch_licenses_names_list(&self) -> Vec<String> {
        self.get_licenses()
            .iter()
            .map(|license| license.format_license_names())
            .collect()
    }
}

pub fn run() -> Result<(), Box<dyn Error>> {
    let licenses = Licenses::from_licenses_file();
    let cli = Cli::parse();
    match &cli.command {
        Commands::List => {
            let licenses_names_list = licenses.fetch_licenses_names_list();
            for license_names in licenses_names_list {
                println!("{}", license_names);
            }
        }
        Commands::Show {
            license,
            user,
            year,
            template,
        } => {
            let license_description_option = licenses.get_license(license);
            if let Some(license_description) = license_description_option {
                if *template {
                    let license_template = license_description.fetch_template();
                    println!("{}", license_template);
                } else {
                    let license = license_description.render_licence(user, year);
                    println!("{}", license);
                }
            } else {
                eprintln!("specified license "{}" not in list", license);
            }
        }
        Commands::Add {
            license,
            user,
            year,
        } => {
            let license_template = util::fetch_license_template(license);
            let license = util::render_licence(license, license_template, user, year);
            let mut license_file = File::create("LICENSE")?;
            license_file.write_all(license.as_bytes())?;
        }
    }
    Ok(())
}
