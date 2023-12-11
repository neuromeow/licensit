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
pub struct LicensePlaceholders {
    author: String,
    year: String,
}

impl LicensePlaceholders {
    pub fn get_author(&self) -> &str {
        &self.author
    }

    pub fn get_year(&self) -> &str {
        &self.year
    }
}

#[derive(Debug, Deserialize)]
pub struct LicenseDescription {
    abbreviation: String,
    name: String,
    template_path: String,
    placeholders: Option<LicensePlaceholders>,
}

impl LicenseDescription {
    fn get_abbreviation(&self) -> &str {
        &self.abbreviation
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_template_path(&self) -> &str {
        &self.template_path
    }

    fn get_placeholders(&self) -> &Option<LicensePlaceholders> {
        &self.placeholders
    }

    pub fn fetch_license_template(&self) -> &str {
        let license_template_relative_path = self.get_template_path();
        let license_template_file = LICENSES_DIR
            .get_file(license_template_relative_path)
            .unwrap();
        let license_template = license_template_file.contents_utf8().unwrap();
        license_template
    }

    pub fn render_licence(&self, license_author: &str, license_year: &u32,) -> String {
        let license_template = self.fetch_license_template();
        let license_placeholders_option = self.get_placeholders();
        if let Some(placeholders) = license_placeholders_option {
            let license_author_placeholder = placeholders.get_author();
            let license_year_placeholder = placeholders.get_year();
            let license = license_template.replace(license_author_placeholder, license_author);
            return license.replace(license_year_placeholder, license_year.to_string().as_str());
        }
        license_template.to_string()
    }
}

#[derive(Debug, Deserialize)]
pub struct LicenseDescriptions {
    pub licenses: Vec<LicenseDescription>,
}

impl LicenseDescriptions {
    pub fn from_licenses_descriptions_file() -> Self {
        let licenses_descriptions_file = LICENSES_DIR.get_file(LICENSES_DESCRIPTIONS_FILE_BASENAME).unwrap();
        let licenses_descriptions_file_content = licenses_descriptions_file.contents_utf8().unwrap();
        serde_yaml::from_str::<LicenseDescriptions>(licenses_descriptions_file_content).unwrap()
    }

    fn get_licenses(&self) -> &Vec<LicenseDescription> {
        &self.licenses
    }

    pub fn get_license_description(&self, license_abbreviation: &str) -> Result<&LicenseDescription, String> {
        for license_description in self.get_licenses() {
            if license_abbreviation == license_description.get_abbreviation() {
                return Ok(license_description);
            }
        }
        Err(format!(
            "specified license not in list {:?}",
            license_abbreviation
        ))
    }

    pub fn render_licenses_list(&self) -> Vec<String> {
        let mut licences_list = Vec::new();
        let license_descriptions = self.get_licenses();
        for license_description in license_descriptions {
            let license_abbreviation = license_description.get_abbreviation();
            let license_name = license_description.get_name();
            let licence_names = format!("{: <12}{}", license_abbreviation, license_name);
            licences_list.push(licence_names);
        }
        licences_list
    }
}

pub fn run() -> Result<(), Box<dyn Error>> {
    let license_descriptions = LicenseDescriptions::from_licenses_descriptions_file();
    let cli = Cli::parse();
    match &cli.command {
        Commands::List => {
            let licenses_list = license_descriptions.render_licenses_list();
            for license in licenses_list {
                println!("{}", license);
            }
        }
        Commands::Show {
            license,
            user,
            year,
            template,
        } => {
            let license_description_result = license_descriptions.get_license_description(license);
            if let Ok(license_description) = license_description_result {
                if *template {
                    let license_template = license_description.fetch_license_template();
                    println!("{}", license_template);
                } else {
                    let license = license_description.render_licence(user, year);
                    println!("{}", license);
                }
            } else {
                eprintln!("{}", license_description_result.unwrap_err());
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
