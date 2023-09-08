use std::error::Error;
use std::fs;

const LICENSES_NAMES: [(&str, &str); 7] = [
    ("agpl-3.0", "GNU Affero General Public License v3.0"),
    ("apache-2.0", "Apache License 2.0"),
    ("gpl-3.0", "GNU General Public License v3.0"),
    ("lgpl-3.0", "GNU Lesser General Public License v3.0"),
    ("mit", "MIT License"),
    ("mpl-2.0", "Mozilla Public License 2.0"),
    ("unlicense", "The Unlicense"),
];

fn print_licence_names_list() {
    for (license_abbreviation, license_name) in LICENSES_NAMES {
        println!("{license_abbreviation: <15} {license_name}");
    }
}

pub fn run() -> Result<(), Box<dyn Error>> {
    print_licence_names_list();
    Ok(())
}
