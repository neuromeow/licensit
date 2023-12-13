use assert_cmd::Command;
use chrono::Datelike;
use serial_test::serial;
use std::fs;
use std::io::Write;

const MIT_LICENSE: &str = "MIT License

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
    let mut mit_license_with_fillers = String::from(MIT_LICENSE);
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

fn create_licensit_list_command() -> Command {
    let mut licensit_list_command = Command::cargo_bin("licensit").unwrap();
    licensit_list_command.arg("list");
    licensit_list_command
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

#[test]
fn licensit_list() {
    create_licensit_list_command().assert().success().stdout(
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

#[test]
fn licensit_show_with_env_variable() {
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

#[test]
fn licensit_show_with_user_option() {
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

#[test]
fn test_licensit_show_with_user_option_and_env_variable() {
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

#[test]
fn test_licensit_show_with_year_option_and_env_variable() {
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

#[test]
fn test_licensit_show_with_user_and_year_options() {
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

#[test]
fn test_licensit_show_with_user_and_year_options_and_env_variable() {
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

#[test]
fn test_licensit_show_with_template_option() {
    create_licensit_show_command()
        .arg("mit")
        .arg("--template")
        .assert()
        .success()
        .stdout(MIT_LICENSE);
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
