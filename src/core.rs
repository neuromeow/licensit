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
            // let license_template = license_descriptions.fetch_license_template(license);
            if *template {
                let license_template = license_descriptions.fetch_license_template(license);
                println!("{}", license_template);
            } else {
                let license = license_descriptions.render_licence(license, user, year);
                println!("{}", license);
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
