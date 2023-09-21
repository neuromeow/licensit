use clap::Parser;
use std::error::Error;
use std::fs::File;
use std::io::Write;

use crate::cli::{Cli, Commands};
use crate::util::*;

pub fn run() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();
    match &cli.command {
        Commands::List => {
            let licences_list = render_licences_list();
            println!("{}", licences_list);
        }
        Commands::Show {
            license,
            user,
            year,
            template,
        } => {
            let license_template = fetch_license_template(license);
            if *template {
                println!("{}", license_template);
            } else {
                let license = render_licence(license, license_template, user, year);
                println!("{}", license);
            }
        }
        Commands::Add {
            license,
            user,
            year,
        } => {
            let license_template = fetch_license_template(license);
            let license = render_licence(license, license_template, user, year);
            let mut license_file = File::create("LICENSE")?;
            license_file.write_all(license.as_bytes())?;
        }
    }
    Ok(())
}
