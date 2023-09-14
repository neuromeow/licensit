use clap::Parser;
use std::collections::HashMap;
use std::error::Error;
use std::fs;

use crate::cli::{Cli, Commands};

const LICENSES_TEMPLATES_PATH: &str = "./licenses/templates";

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

fn get_license_template(filepath: &str) -> Result<String, Box<dyn Error>> {
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
            let license_template_filepath = format!("{}/{}", LICENSES_TEMPLATES_PATH, license);
            let license_template = get_license_template(&license_template_filepath).unwrap();
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
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_license_content() {
        let unlicense_license_filepath = "./licenses/templates/unlicense";
        let unlicense_license_expected_content =
            "This is free and unencumbered software released into the public domain.

Anyone is free to copy, modify, publish, use, compile, sell, or
distribute this software, either in source code form or as a compiled
binary, for any purpose, commercial or non-commercial, and by any
means.

In jurisdictions that recognize copyright laws, the author or authors
of this software dedicate any and all copyright interest in the
software to the public domain. We make this dedication for the benefit
of the public at large and to the detriment of our heirs and
successors. We intend this dedication to be an overt act of
relinquishment in perpetuity of all present and future rights to this
software under copyright law.

THE SOFTWARE IS PROVIDED \"AS IS\", WITHOUT WARRANTY OF ANY KIND,
EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.
IN NO EVENT SHALL THE AUTHORS BE LIABLE FOR ANY CLAIM, DAMAGES OR
OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
OTHER DEALINGS IN THE SOFTWARE.

For more information, please refer to <https://unlicense.org>
";
        assert_eq!(
            get_license_template(unlicense_license_filepath).unwrap(),
            unlicense_license_expected_content
        );
    }
}
