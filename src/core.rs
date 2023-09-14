use clap::Parser;
use std::error::Error;

use crate::cli::{Cli, Commands};
use crate::util::*;

const LICENSES_TEMPLATES_PATH: &str = "./licenses/templates";

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
