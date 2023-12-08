use clap::Parser;
use std::error::Error;
use std::fs::File;
use std::io::Write;

use crate::cli::{Cli, Commands};
use crate::util;
use crate::license_renderers::LicenseDescriptions;

pub fn run() -> Result<(), Box<dyn Error>> {
    let license_descriptions = LicenseDescriptions::from_licenses_descriptions_file();
    let cli = Cli::parse();
    match &cli.command {
        Commands::List => {
            let licences_list = license_descriptions.render_licences_list();
            println!("{}", licences_list);
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
