use include_dir::{include_dir, Dir};
use std::collections::HashMap;

static LICENSES_DIR: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/licenses");

pub const LICENSES_ABBREVIATIONS: [&str; 7] = [
    "agpl-3.0",
    "apache-2.0",
    "gpl-3.0",
    "lgpl-3.0",
    "mit",
    "mpl-2.0",
    "unlicense",
];

const LICENSES_NAMES: [&str; 7] = [
    "GNU Affero General Public License v3.0",
    "Apache License 2.0",
    "GNU General Public License v3.0",
    "GNU Lesser General Public License v3.0",
    "MIT License",
    "Mozilla Public License 2.0",
    "The Unlicense",
];

pub fn print_licence_names_list() {
    for (license_abbreviation, license_name) in
        LICENSES_ABBREVIATIONS.into_iter().zip(LICENSES_NAMES)
    {
        println!("{license_abbreviation: <15} {license_name}");
    }
}

pub fn fetch_license_template(license_name: &str) -> &str {
    let license_template_relative_path = format!("templates/{}", license_name);
    // It is necessary that there are license template files with names
    // that are present in the list with abbreviations of available licenses.
    let license_template_file = LICENSES_DIR
        .get_file(license_template_relative_path)
        .unwrap();
    let license_template = license_template_file.contents_utf8().unwrap();
    license_template
}

pub fn render_licence(
    license_name: &str,
    license_template: &str,
    license_author: &str,
    license_year: &u16,
) -> String {
    // Not all available licenses have placeholders.
    let licenses_placeholders = HashMap::from([
        ("agpl-3.0", ("<name of author>", "<year>")),
        ("apache-2.0", ("[name of copyright owner]", "[yyyy]")),
        ("gpl-3.0", ("<name of author>", "<year>")),
        ("mit", ("[fullname]", "[year]")),
    ]);
    let license_placeholders = licenses_placeholders.get(license_name).copied();
    if let Some(placeholders) = license_placeholders {
        let license = license_template.replace(placeholders.0, license_author);
        license.replace(placeholders.1, license_year.to_string().as_str())
    } else {
        license_template.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fetch_license_template() {
        let unlicense_license_template_expected_content =
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
            fetch_license_template("unlicense"),
            unlicense_license_template_expected_content
        );
    }
}
