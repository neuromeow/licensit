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
            print_licence_names_list();
        }
        Commands::Show {
            license,
            user,
            year,
            template,
        } => {
            let license_template = get_license_template(license).unwrap();
            if *template {
                println!("{}", license_template);
            } else {
                let license_author = if let Some(author) = user {
                    author
                } else {
                    "user"
                };
                let license = render_licence(license, &license_template, license_author, year);
                println!("{}", license);
            }
        }
        Commands::Add {
            license,
            user,
            year,
        } => {
            let license_template = get_license_template(license).unwrap();
            let license_author = if let Some(author) = user {
                author
            } else {
                "user"
            };
            let license = render_licence(license, &license_template, license_author, year);
            let mut license_file = File::create("LICENSE")?;
            license_file.write_all(license.as_bytes())?;
        }
    }
    Ok(())
}
