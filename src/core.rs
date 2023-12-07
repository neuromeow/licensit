use clap::Parser;
use std::error::Error;
use std::fs::File;
use std::io::Write;

use crate::cli::{Cli, Commands};
use crate::{license_renderers, util};

pub fn run() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();
    match &cli.command {
        Commands::List => {
            let license_descriptions = license_renderers::load_license_descriptions();
            // let licences_list = license_renderers::render_licences_list_new(license_descriptions);
            let licences_list = license_descriptions.render_licences_list();
            println!("{}", licences_list);
        }
        Commands::Show {
            license,
            user,
            year,
            template,
        } => {
            let license_template = license_renderers::fetch_license_template_new(license);
            if *template {
                println!("{}", license_template);
            } else {
                let license =
                    license_renderers::render_licence_new(license, license_template, user, year);
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
