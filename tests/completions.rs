use assert_cmd::Command;
use chrono::Datelike;
use serial_test::serial;
use std::fs;
use std::io::Write;

const UNLICENSE_LICENSE: &str =
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

const MIT_LICENSE_TEMPLATE: &str = "MIT License

Copyright (c) [year] [fullname]

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

fn render_mit_license_with_fillers(
    fullname_filler: Option<&str>,
    year_filler: Option<&str>,
) -> String {
    let mut mit_license_with_fillers = String::from(MIT_LICENSE_TEMPLATE);
    if let Some(filler) = fullname_filler {
        mit_license_with_fillers = mit_license_with_fillers.replace("[fullname]", filler);
    };
    if let Some(filler) = year_filler {
        mit_license_with_fillers = mit_license_with_fillers.replace("[year]", filler);
    } else {
        let current_year = chrono::Utc::now().year().to_string();
        mit_license_with_fillers =
            mit_license_with_fillers.replace("[year]", current_year.as_str());
    };
    mit_license_with_fillers
}

fn read_project_license() -> Result<String, Box<dyn std::error::Error>> {
    let project_license = fs::read_to_string("LICENSE")?;
    Ok(project_license)
}

fn overwrite_project_license(content: String) -> Result<(), Box<dyn std::error::Error>> {
    let mut project_license = fs::OpenOptions::new()
        .write(true)
        .truncate(true)
        .open("LICENSE")?;
    project_license.write(content.as_bytes())?;
    project_license.flush()?;
    Ok(())
}

fn create_licensit_show_command() -> Command {
    let mut licensit_show_command = Command::cargo_bin("licensit").unwrap();
    licensit_show_command.arg("show");
    licensit_show_command
}

fn create_licensit_add_command() -> Command {
    let mut licensit_add_command = Command::cargo_bin("licensit").unwrap();
    licensit_add_command.arg("add");
    licensit_add_command
}

// Simulated command:
// `licensit list`
#[ignore]
#[test]
fn test_licensit_list() {
    let mut cmd = Command::cargo_bin("licensit").unwrap();
    cmd.arg("list");
    cmd.assert().success().stdout(
        "\
agpl-3.0    GNU Affero General Public License v3.0
apache-2.0  Apache License 2.0
gpl-3.0     GNU General Public License v3.0
lgpl-3.0    GNU Lesser General Public License v3.0
mit         MIT License
mpl-2.0     Mozilla Public License 2.0
unlicense   The Unlicense
",
    );
}

// Only for Unlicense License as for license without placeholders.
// Simulated command:
// `licensit show unlicense`
#[test]
fn test_licensit_show_for_license_without_placeholders() {
    create_licensit_show_command()
        .arg("unlicense")
        .assert()
        .success()
        .stdout(UNLICENSE_LICENSE);
}

// Only for Unlicense License as for license without placeholders.
// Simulated command:
// `LICENSE_AUTHOR=license_author_env_variable licensit show unlicense`
#[test]
fn test_licensit_show_with_env_variable_for_license_without_placeholders() {
    create_licensit_show_command()
        .arg("unlicense")
        .env("LICENSE_AUTHOR", "license_author_env_variable")
        .assert()
        .success()
        .stdout(UNLICENSE_LICENSE);
}

// Only for Unlicense License as for license without placeholders.
// Simulated command:
// `licensit show unlicense --user license_author_passed_var`
#[test]
fn test_licensit_show_with_user_option_for_license_without_placeholders() {
    create_licensit_show_command()
        .arg("unlicense")
        .arg("--user=license_author_passed_var")
        .assert()
        .success()
        .stdout(UNLICENSE_LICENSE);
}

// Only for Unlicense License as for license without placeholders.
// Simulated command:
// `LICENSE_AUTHOR=license_author_env_variable licensit show unlicense --user license_author_passed_var`
#[test]
fn test_licensit_show_with_user_option_and_env_variable_for_license_without_placeholders() {
    create_licensit_show_command()
        .arg("unlicense")
        .arg("--user=license_author_passed_var")
        .env("LICENSE_AUTHOR", "license_author_env_variable")
        .assert()
        .success()
        .stdout(UNLICENSE_LICENSE);
}

// Only for Unlicense License as for license without placeholders.
// Simulated command:
// `licensit show unlicense --year 2023`
#[test]
fn test_licensit_show_with_year_option_for_license_without_placeholders() {
    create_licensit_show_command()
        .arg("unlicense")
        .arg("--year=2023")
        .assert()
        .success()
        .stdout(UNLICENSE_LICENSE);
}

// Only for Unlicense License as for license without placeholders.
// Simulated command:
// `licensit show unlicense --user license_author_passed_var --year 2023`
#[test]
fn test_licensit_show_with_user_and_year_options_for_license_without_placeholders() {
    create_licensit_show_command()
        .arg("unlicense")
        .arg("--user=license_author_passed_var")
        .arg("--year=2023")
        .assert()
        .success()
        .stdout(UNLICENSE_LICENSE);
}

// Only for Unlicense License as for license without placeholders.
// Simulated command:
// `licensit show unlicense --template`
#[test]
fn test_licensit_show_with_template_option_for_license_without_placeholders() {
    create_licensit_show_command()
        .arg("unlicense")
        .arg("--template")
        .assert()
        .success()
        .stdout(UNLICENSE_LICENSE);
}

// Only for Unlicense License as for license without placeholders.
// Simulated command:
// `licensit show unlicense --user license_author_passed_var --template`
#[test]
fn test_licensit_show_with_user_and_template_options_for_license_without_placeholders() {
    create_licensit_show_command()
        .arg("unlicense")
        .arg("--user=license_author_passed_var")
        .arg("--template")
        .assert()
        .failure()
        .stderr(
            "error: the argument '--user <USER>' cannot be used with '--template'

Usage: licensit show --user <USER> <LICENSE>

For more information, try '--help'.
",
        );
}

// Only for Unlicense License as for license without placeholders.
// Simulated command:
// `licensit show unlicense --year 2023 --template`
#[test]
fn test_licensit_show_with_year_and_template_options_for_license_without_placeholders() {
    create_licensit_show_command()
        .arg("unlicense")
        .arg("--year=2023")
        .arg("--template")
        .assert()
        .failure()
        .stderr(
            "error: the argument '--year <YEAR>' cannot be used with '--template'

Usage: licensit show --year <YEAR> <LICENSE>

For more information, try '--help'.
",
        );
}

// Only for Unlicense License as for license without placeholders.
// Simulated command:
// `licensit show unlicense --user license_author_passed_var --year 2023 --template`
#[test]
fn test_licensit_show_with_all_options_for_license_without_placeholders() {
    create_licensit_show_command()
        .arg("unlicense")
        .arg("--user=license_author_passed_var")
        .arg("--year=2023")
        .arg("--template")
        .assert()
        .failure()
        .stderr(
            "error: the argument '--user <USER>' cannot be used with '--template'

Usage: licensit show --user <USER> --year <YEAR> <LICENSE>

For more information, try '--help'.
",
        );
}

// Only for MIT License as for license with placeholders.
// Simulated command:
// `LICENSE_AUTHOR=license_author_env_variable licensit show mit`
#[test]
fn test_licensit_show_with_env_variable_for_license_with_placeholders() {
    let license_author_env_variable = "license_author_env_variable";
    create_licensit_show_command()
        .arg("mit")
        .env("LICENSE_AUTHOR", license_author_env_variable)
        .assert()
        .success()
        .stdout(render_mit_license_with_fillers(
            Some(license_author_env_variable),
            None,
        ));
}

// Only for MIT License as for license with placeholders.
// Simulated command:
// `licensit show mit --user license_author_passed_var`
#[test]
fn test_licensit_show_with_user_option_for_license_with_placeholders() {
    let license_author_passed_var = "license_author_passed_var";
    let user_option_with_value = format!("--user={}", license_author_passed_var);
    create_licensit_show_command()
        .arg("mit")
        .arg(user_option_with_value)
        .assert()
        .success()
        .stdout(render_mit_license_with_fillers(
            Some(license_author_passed_var),
            None,
        ));
}

// Only for MIT License as for license with placeholders.
// Simulated command:
// `LICENSE_AUTHOR=license_author_env_variable licensit show mit --user license_author_passed_var`
#[test]
fn test_licensit_show_with_user_option_and_env_variable_for_license_with_placeholders() {
    let license_author_passed_var = "license_author_passed_var";
    let user_option_with_value = format!("--user={}", license_author_passed_var);
    let license_author_env_variable = "license_author_env_variable";
    create_licensit_show_command()
        .arg("mit")
        .arg(user_option_with_value)
        .env("LICENSE_AUTHOR", license_author_env_variable)
        .assert()
        .success()
        .stdout(render_mit_license_with_fillers(
            Some(license_author_passed_var),
            None,
        ));
}

// Only for MIT License as for license with placeholders.
// Simulated command:
// `LICENSE_AUTHOR=license_author_env_variable licensit show mit --year 2023`
#[test]
fn test_licensit_show_with_year_option_and_env_variable_for_license_with_placeholders() {
    let license_author_env_variable = "license_author_env_variable";
    let year_passed_var = "2023";
    let year_option_with_value = format!("--year={}", year_passed_var);
    create_licensit_show_command()
        .arg("mit")
        .arg(year_option_with_value)
        .env("LICENSE_AUTHOR", license_author_env_variable)
        .assert()
        .success()
        .stdout(render_mit_license_with_fillers(
            Some(license_author_env_variable),
            Some(year_passed_var),
        ));
}

// Only for MIT License as for license with placeholders.
// Simulated command:
// `licensit show mit --user license_author_passed_var --year 2023`
#[test]
fn test_licensit_show_with_user_and_year_options_for_license_with_placeholders() {
    let license_author_passed_var = "license_author_passed_var";
    let user_option_with_value = format!("--user={}", license_author_passed_var);
    let year_passed_var = "2023";
    let year_option_with_value = format!("--year={}", year_passed_var);
    create_licensit_show_command()
        .arg("mit")
        .arg(user_option_with_value)
        .arg(year_option_with_value)
        .assert()
        .success()
        .stdout(render_mit_license_with_fillers(
            Some(license_author_passed_var),
            Some(year_passed_var),
        ));
}

// Only for MIT License as for license with placeholders.
// Simulated command:
// `LICENSE_AUTHOR=license_author_env_variable licensit show mit --user license_author_passed_var --year 2023`
#[test]
fn test_licensit_show_with_user_and_year_options_and_env_variable_for_license_with_placeholders() {
    let license_author_passed_var = "license_author_passed_var";
    let user_option_with_value = format!("--user={}", license_author_passed_var);
    let license_author_env_variable = "license_author_env_variable";
    let year_passed_var = "2023";
    let year_option_with_value = format!("--year={}", year_passed_var);
    create_licensit_show_command()
        .arg("mit")
        .arg(user_option_with_value)
        .arg(year_option_with_value)
        .env("LICENSE_AUTHOR", license_author_env_variable)
        .assert()
        .success()
        .stdout(render_mit_license_with_fillers(
            Some(license_author_passed_var),
            Some(year_passed_var),
        ));
}

// Only for MIT License as for license with placeholders.
// Simulated command:
// `licensit show mit --template`
#[test]
fn test_licensit_show_with_template_option_for_license_with_placeholders() {
    create_licensit_show_command()
        .arg("mit")
        .arg("--template")
        .assert()
        .success()
        .stdout(MIT_LICENSE_TEMPLATE);
}

// Only for MIT License as for license with placeholders.
// Simulated command:
// `LICENSE_AUTHOR=license_author_env_variable licensit add mit`
#[test]
#[serial]
fn test_licensit_add_with_env_variable_for_license_with_placeholders() {
    let original_project_license = read_project_license().unwrap();
    let license_author_env_variable = "license_author_env_variable";
    create_licensit_add_command()
        .arg("mit")
        .env("LICENSE_AUTHOR", license_author_env_variable)
        .assert()
        .success();
    let mut modified_project_license = read_project_license().unwrap();
    modified_project_license.push('\n');
    overwrite_project_license(original_project_license).unwrap();
    assert_eq!(
        modified_project_license,
        render_mit_license_with_fillers(Some(license_author_env_variable), None)
    );
}

// Only for MIT License as for license with placeholders.
// Simulated command:
// `licensit add mit --user license_author_passed_var`
#[test]
#[serial]
fn test_licensit_add_with_user_option_for_license_with_placeholders() {
    let original_project_license = read_project_license().unwrap();
    let license_author_passed_var = "license_author_passed_var";
    let user_option_with_value = format!("--user={}", license_author_passed_var);
    create_licensit_add_command()
        .arg("mit")
        .arg(user_option_with_value)
        .assert()
        .success();
    let mut modified_project_license = read_project_license().unwrap();
    modified_project_license.push('\n');
    overwrite_project_license(original_project_license).unwrap();
    assert_eq!(
        modified_project_license,
        render_mit_license_with_fillers(Some(license_author_passed_var), None)
    );
}

// Only for MIT License as for license with placeholders.
// Simulated command:
// `LICENSE_AUTHOR=license_author_env_variable licensit add mit --user license_author_passed_var`
#[test]
#[serial]
fn test_licensit_add_with_user_option_and_env_variable_for_license_with_placeholders() {
    let original_project_license = read_project_license().unwrap();
    let license_author_passed_var = "license_author_passed_var";
    let user_option_with_value = format!("--user={}", license_author_passed_var);
    let license_author_env_variable = "license_author_env_variable";
    create_licensit_add_command()
        .arg("mit")
        .arg(user_option_with_value)
        .env("LICENSE_AUTHOR", license_author_env_variable)
        .assert()
        .success();
    let mut modified_project_license = read_project_license().unwrap();
    modified_project_license.push('\n');
    overwrite_project_license(original_project_license).unwrap();
    assert_eq!(
        modified_project_license,
        render_mit_license_with_fillers(Some(license_author_passed_var), None)
    );
}

// Only for MIT License as for license with placeholders.
// Simulated command:
// `LICENSE_AUTHOR=license_author_env_variable licensit add mit --year 2023`
#[test]
#[serial]
fn test_licensit_add_with_year_option_and_env_variable_for_license_with_placeholders() {
    let original_project_license = read_project_license().unwrap();
    let license_author_env_variable = "license_author_env_variable";
    let year_passed_var = "2023";
    let year_option_with_value = format!("--year={}", year_passed_var);
    create_licensit_add_command()
        .arg("mit")
        .arg(year_option_with_value)
        .env("LICENSE_AUTHOR", license_author_env_variable)
        .assert()
        .success();
    let mut modified_project_license = read_project_license().unwrap();
    modified_project_license.push('\n');
    overwrite_project_license(original_project_license).unwrap();
    assert_eq!(
        modified_project_license,
        render_mit_license_with_fillers(Some(license_author_env_variable), Some(year_passed_var))
    );
}

// Only for MIT License as for license with placeholders.
// Simulated command:
// `licensit add mit --user license_author_passed_var --year 2023`
#[test]
#[serial]
fn test_licensit_add_with_user_and_year_options_for_license_with_placeholders() {
    let original_project_license = read_project_license().unwrap();
    let license_author_passed_var = "license_author_passed_var";
    let user_option_with_value = format!("--user={}", license_author_passed_var);
    let year_passed_var = "2023";
    let year_option_with_value = format!("--year={}", year_passed_var);
    create_licensit_add_command()
        .arg("mit")
        .arg(user_option_with_value)
        .arg(year_option_with_value)
        .assert()
        .success();
    let mut modified_project_license = read_project_license().unwrap();
    modified_project_license.push('\n');
    overwrite_project_license(original_project_license).unwrap();
    assert_eq!(
        modified_project_license,
        render_mit_license_with_fillers(Some(license_author_passed_var), Some(year_passed_var))
    );
}

// Only for MIT License as for license with placeholders.
// Simulated command:
// `LICENSE_AUTHOR=license_author_env_variable licensit add mit --user license_author_passed_var --year 2023`
#[test]
#[serial]
fn test_licensit_add_with_user_and_year_options_and_env_for_license_with_placeholders() {
    let original_project_license = read_project_license().unwrap();
    let license_author_passed_var = "license_author_passed_var";
    let user_option_with_value = format!("--user={}", license_author_passed_var);
    let license_author_env_variable = "license_author_env_variable";
    let year_passed_var = "2023";
    let year_option_with_value = format!("--year={}", year_passed_var);
    create_licensit_add_command()
        .arg("mit")
        .arg(user_option_with_value)
        .arg(year_option_with_value)
        .env("LICENSE_AUTHOR", license_author_env_variable)
        .assert()
        .success();
    let mut modified_project_license = read_project_license().unwrap();
    modified_project_license.push('\n');
    overwrite_project_license(original_project_license).unwrap();
    assert_eq!(
        modified_project_license,
        render_mit_license_with_fillers(Some(license_author_passed_var), Some(year_passed_var))
    );
}
