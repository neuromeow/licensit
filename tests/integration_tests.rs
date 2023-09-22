use assert_cmd::Command;
use chrono::Datelike;

const UNLICENSE_LICENSE_CONTENT: &str =
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

const MIT_LICENSE_EXAMPLE_CONTENT: &str = "MIT License

Copyright (c) [year] example_license_author

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the \"Software\"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED \"AS IS\", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.

";

fn create_licensit_show_command() -> Command {
    let mut licensit_show_command = Command::cargo_bin("licensit").unwrap();
    licensit_show_command.arg("show");
    licensit_show_command
}

#[test]
fn test_licensit_list() {
    let mut cmd = Command::cargo_bin("licensit").unwrap();
    cmd.arg("list");
    cmd.assert().stdout(
        "\
agpl-3.0       GNU Affero General Public License v3.0
apache-2.0     Apache License 2.0
gpl-3.0        GNU General Public License v3.0
lgpl-3.0       GNU Lesser General Public License v3.0
mit            MIT License
mpl-2.0        Mozilla Public License 2.0
unlicense      The Unlicense
",
    );
}

#[test]
fn test_licensit_show_for_license_without_placeholders() {
    create_licensit_show_command()
        .arg("unlicense")
        .assert()
        .stdout(UNLICENSE_LICENSE_CONTENT);
}

#[test]
fn test_licensit_show_with_user_option_for_license_without_placeholders() {
    create_licensit_show_command()
        .arg("unlicense")
        .arg("--user=license_author")
        .assert()
        .stdout(UNLICENSE_LICENSE_CONTENT);
}

#[test]
fn test_licensit_show_with_year_option_for_license_without_placeholders() {
    create_licensit_show_command()
        .arg("unlicense")
        .arg("--year=2023")
        .assert()
        .stdout(UNLICENSE_LICENSE_CONTENT);
}

#[test]
fn test_licensit_show_with_user_and_year_options_for_license_without_placeholders() {
    create_licensit_show_command()
        .arg("unlicense")
        .arg("--user=license_author")
        .arg("--year=2023")
        .assert()
        .stdout(UNLICENSE_LICENSE_CONTENT);
}

#[test]
fn test_licensit_show_with_template_option_for_license_without_placeholders() {
    create_licensit_show_command()
        .arg("unlicense")
        .arg("--template")
        .assert()
        .stdout(UNLICENSE_LICENSE_CONTENT);
}

#[test]
fn test_licensit_show_with_user_and_template_options_for_license_without_placeholders() {
    create_licensit_show_command()
        .arg("unlicense")
        .arg("--user=license_author")
        .arg("--template")
        .assert()
        .failure();
}

#[test]
fn test_licensit_show_with_year_and_template_options_for_license_without_placeholders() {
    create_licensit_show_command()
        .arg("unlicense")
        .arg("--year=2023")
        .arg("--template")
        .assert()
        .failure();
}

#[test]
fn test_licensit_show_with_all_options_for_license_without_placeholders() {
    create_licensit_show_command()
        .arg("unlicense")
        .arg("--user=license_author")
        .arg("--year=2023")
        .arg("--template")
        .assert()
        .failure();
}

#[test]
fn test_licensit_show_with_user_option_for_license_with_placeholders() {
    let current_year = chrono::Utc::now().year().to_string();
    create_licensit_show_command()
        .arg("mit")
        .arg("--user=example_license_author")
        .assert()
        .stdout(MIT_LICENSE_EXAMPLE_CONTENT.replace("[year]", current_year.as_str()));
}

#[test]
fn test_licensit_show_with_user_and_year_options_for_license_with_placeholders() {
    let year_example = "2023";
    let year_option_with_year_example_value = format!("--year={}", year_example);
    create_licensit_show_command()
        .arg("mit")
        .arg("--user=example_license_author")
        .arg(year_option_with_year_example_value)
        .assert()
        .stdout(MIT_LICENSE_EXAMPLE_CONTENT.replace("[year]", year_example));
}

#[test]
fn test_licensit_show_with_template_option_for_license_with_placeholders() {
    create_licensit_show_command()
        .arg("mit")
        .arg("--template")
        .assert()
        .stdout(MIT_LICENSE_EXAMPLE_CONTENT.replace("example_license_author", "[fullname]"));
}
