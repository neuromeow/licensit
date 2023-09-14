use std::collections::HashMap;
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

pub fn print_licence_names_list() {
    for (license_abbreviation, license_name) in LICENSES_NAMES {
        println!("{license_abbreviation: <15} {license_name}");
    }
}

pub fn get_license_template(filepath: &str) -> Result<String, Box<dyn Error>> {
    let license_content = fs::read_to_string(filepath)?;
    Ok(license_content)
}

pub fn render_licence(
    license_name: &str,
    license_template: &str,
    license_author: &str,
    license_year: &u16,
) -> String {
    let licenses = HashMap::from([
        ("agpl-3.0", ("<name of author>", "<year>")),
        ("apache-2.0", ("[name of copyright owner]", "[yyyy]")),
        ("gpl-3.0", ("<name of author>", "<year>")),
        ("mit", ("[fullname]", "[year]")),
    ]);
    let license_placeholders = licenses.get(license_name).copied();
    if let Some(placeholders) = license_placeholders {
        let custom_license = license_template.replace(placeholders.0, license_author);
        custom_license.replace(placeholders.1, license_year.to_string().as_str())
    } else {
        license_template.to_string()
    }
}
