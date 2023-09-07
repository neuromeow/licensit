use std::collections::HashMap;
use std::error::Error;
use std::fs;

#[allow(dead_code)]
pub fn print_licence_template_files_list() {
    let licence_template_files = fs::read_dir("licenses/templates").unwrap();
    for licence_template_file in licence_template_files {
        println!(
            "{}",
            licence_template_file.unwrap().file_name().to_str().unwrap()
        )
    }
}

pub fn print_licences_list() {
    let licenses_names = HashMap::from([
        ("agpl-3.0", "GNU Affero General Public License v3.0"),
        ("apache-2.0", "Apache License 2.0"),
        ("gpl-3.0", "GNU General Public License v3.0"),
        ("lgpl-3.0", "GNU Lesser General Public License v3.0"),
        ("mit", "MIT License"),
        ("mpl-2.0", "Mozilla Public License 2.0"),
        ("unlicense", "The Unlicense"),
    ]);
    let license_template_files = fs::read_dir("licenses/templates").unwrap();
    for license_template_file in license_template_files {
        let license_template_file_basename = license_template_file.unwrap().file_name();
        let license_full_name = licenses_names
            .get(license_template_file_basename.to_str().unwrap())
            .unwrap();
        println!(
            "{: <15} {}",
            license_template_file_basename.to_str().unwrap(),
            license_full_name
        );
    }
}

pub fn run() -> Result<(), Box<dyn Error>> {
    print_licences_list();
    Ok(())
}
