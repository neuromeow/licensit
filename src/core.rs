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

    // TODO: Method needs to be refactored
    fn get_license_description(&self, license_abbreviation: &str) -> Result<&License, String> {
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

    // TODO: Method needs to be refactored
    fn render_licenses_list(&self) -> Vec<String> {
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
    let licenses = Licenses::from_licenses_file();
    let cli = Cli::parse();
    match &cli.command {
        Commands::List => {
            let licenses_list = licenses.render_licenses_list();
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
            let license_description_result = licenses.get_license_description(license);
            if let Ok(license_description) = license_description_result {
                if *template {
                    let license_template = license_description.fetch_template();
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
