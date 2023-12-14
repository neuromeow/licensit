use assert_cmd::Command;
use chrono::Datelike;
use serial_test::serial;
use std::fs;
use std::path::{Path, PathBuf};
use tempfile::TempDir;

const USER_OPTION_VALUE: &str = "user_option_value";
const YEAR_OPTION_VALUE: &str = "2023";
const LICENSE_AUTHOR_ENV_VARIABLE_NAME: &str = "LICENSE_AUTHOR";
const LICENSE_AUTHOR_ENV_VARIABLE_VALUE: &str = "license_author_env_variable_value";

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
const MIT_LICENSE_NAME: &str = "mit";

pub struct TempDirContext {
    original_dir: PathBuf,
    temp_dir: TempDir,
}

impl TempDirContext {
    pub fn new() -> Self {
        let original_dir = std::env::current_dir().unwrap();
        let temp_dir = tempfile::tempdir().unwrap();
        std::env::set_current_dir(temp_dir.path()).unwrap();
        TempDirContext {
            original_dir,
            temp_dir,
        }
    }

    pub fn path(&self) -> &Path {
        self.temp_dir.path()
    }
}

impl Drop for TempDirContext {
    fn drop(&mut self) {
        std::env::set_current_dir(&self.original_dir).unwrap();
    }
}

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
fn licensit_show_with_user_option() {
    let user_option_with_value = format!("--user={}", USER_OPTION_VALUE);
    create_licensit_show_command()
        .arg(MIT_LICENSE_NAME)
        .arg(user_option_with_value)
        .assert()
        .success()
        .stdout(render_mit_license_with_fillers(
            Some(USER_OPTION_VALUE),
            None,
        ));
}

#[test]
fn licensit_show_with_env_variable() {
    create_licensit_show_command()
        .arg(MIT_LICENSE_NAME)
        .env(
            LICENSE_AUTHOR_ENV_VARIABLE_NAME,
            LICENSE_AUTHOR_ENV_VARIABLE_VALUE,
        )
        .assert()
        .success()
        .stdout(render_mit_license_with_fillers(
            Some(LICENSE_AUTHOR_ENV_VARIABLE_VALUE),
            None,
        ));
}

#[test]
fn licensit_show_with_user_option_and_env_variable() {
    let user_option_with_value = format!("--user={}", USER_OPTION_VALUE);
    create_licensit_show_command()
        .arg(MIT_LICENSE_NAME)
        .arg(user_option_with_value)
        .env(
            LICENSE_AUTHOR_ENV_VARIABLE_NAME,
            LICENSE_AUTHOR_ENV_VARIABLE_VALUE,
        )
        .assert()
        .success()
        .stdout(render_mit_license_with_fillers(
            Some(USER_OPTION_VALUE),
            None,
        ));
}

#[test]
fn licensit_show_with_user_and_year_options() {
    let user_option_with_value = format!("--user={}", USER_OPTION_VALUE);
    let year_option_with_value = format!("--year={}", YEAR_OPTION_VALUE);
    create_licensit_show_command()
        .arg(MIT_LICENSE_NAME)
        .arg(user_option_with_value)
        .arg(year_option_with_value)
        .assert()
        .success()
        .stdout(render_mit_license_with_fillers(
            Some(USER_OPTION_VALUE),
            Some(YEAR_OPTION_VALUE),
        ));
}

#[test]
fn licensit_show_with_year_option_and_env_variable() {
    let year_option_with_value = format!("--year={}", YEAR_OPTION_VALUE);
    create_licensit_show_command()
        .arg(MIT_LICENSE_NAME)
        .arg(year_option_with_value)
        .env(
            LICENSE_AUTHOR_ENV_VARIABLE_NAME,
            LICENSE_AUTHOR_ENV_VARIABLE_VALUE,
        )
        .assert()
        .success()
        .stdout(render_mit_license_with_fillers(
            Some(LICENSE_AUTHOR_ENV_VARIABLE_VALUE),
            Some(YEAR_OPTION_VALUE),
        ));
}

#[test]
fn licensit_show_with_user_and_year_options_and_env_variable() {
    let user_option_with_value = format!("--user={}", USER_OPTION_VALUE);
    let year_option_with_value = format!("--year={}", YEAR_OPTION_VALUE);
    create_licensit_show_command()
        .arg(MIT_LICENSE_NAME)
        .arg(user_option_with_value)
        .arg(year_option_with_value)
        .env(
            LICENSE_AUTHOR_ENV_VARIABLE_NAME,
            LICENSE_AUTHOR_ENV_VARIABLE_VALUE,
        )
        .assert()
        .success()
        .stdout(render_mit_license_with_fillers(
            Some(USER_OPTION_VALUE),
            Some(YEAR_OPTION_VALUE),
        ));
}

#[test]
fn licensit_show_with_template_option() {
    create_licensit_show_command()
        .arg(MIT_LICENSE_NAME)
        .arg("--template")
        .assert()
        .success()
        .stdout(MIT_LICENSE);
}

#[test]
fn licensit_show_with_user_and_template_options() {
    let user_option_with_value = format!("--user={}", USER_OPTION_VALUE);
    create_licensit_show_command()
        .arg(MIT_LICENSE_NAME)
        .arg(user_option_with_value)
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

#[test]
fn licensit_show_with_year_and_template_options() {
    let year_option_with_value = format!("--year={}", YEAR_OPTION_VALUE);
    create_licensit_show_command()
        .arg(MIT_LICENSE_NAME)
        .arg(year_option_with_value)
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

#[test]
fn licensit_show_with_all_options() {
    let user_option_with_value = format!("--user={}", USER_OPTION_VALUE);
    let year_option_with_value = format!("--year={}", YEAR_OPTION_VALUE);
    create_licensit_show_command()
        .arg(MIT_LICENSE_NAME)
        .arg(user_option_with_value)
        .arg(year_option_with_value)
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

#[test]
#[serial]
fn licensit_add_with_user_option() {
    let temp_dir_context = TempDirContext::new();
    let user_option_with_value = format!("--user={}", USER_OPTION_VALUE);
    create_licensit_add_command()
        .arg(MIT_LICENSE_NAME)
        .arg(user_option_with_value)
        .assert()
        .success();
    let added_license_file_content =
        fs::read_to_string(temp_dir_context.path().join("LICENSE")).unwrap();
    assert_eq!(
        added_license_file_content,
        render_mit_license_with_fillers(Some(USER_OPTION_VALUE), None)
    );
}

#[test]
#[serial]
fn licensit_add_with_env_variable() {
    let temp_dir_context = TempDirContext::new();
    create_licensit_add_command()
        .arg(MIT_LICENSE_NAME)
        .env(
            LICENSE_AUTHOR_ENV_VARIABLE_NAME,
            LICENSE_AUTHOR_ENV_VARIABLE_VALUE,
        )
        .assert()
        .success();
    let added_license_file_content =
        fs::read_to_string(temp_dir_context.path().join("LICENSE")).unwrap();
    assert_eq!(
        added_license_file_content,
        render_mit_license_with_fillers(Some(LICENSE_AUTHOR_ENV_VARIABLE_VALUE), None)
    );
}

#[test]
#[serial]
fn licensit_add_with_user_option_and_env_variable() {
    let temp_dir_context = TempDirContext::new();
    let user_option_with_value = format!("--user={}", USER_OPTION_VALUE);
    create_licensit_add_command()
        .arg(MIT_LICENSE_NAME)
        .arg(user_option_with_value)
        .env(
            LICENSE_AUTHOR_ENV_VARIABLE_NAME,
            LICENSE_AUTHOR_ENV_VARIABLE_VALUE,
        )
        .assert()
        .success();
    let added_license_file_content =
        fs::read_to_string(temp_dir_context.path().join("LICENSE")).unwrap();
    assert_eq!(
        added_license_file_content,
        render_mit_license_with_fillers(Some(USER_OPTION_VALUE), None)
    );
}

#[test]
#[serial]
fn licensit_add_with_user_and_year_options() {
    let temp_dir_context = TempDirContext::new();
    let user_option_with_value = format!("--user={}", USER_OPTION_VALUE);
    let year_option_with_value = format!("--year={}", YEAR_OPTION_VALUE);
    create_licensit_add_command()
        .arg(MIT_LICENSE_NAME)
        .arg(user_option_with_value)
        .arg(year_option_with_value)
        .assert()
        .success();
    let added_license_file_content =
        fs::read_to_string(temp_dir_context.path().join("LICENSE")).unwrap();
    assert_eq!(
        added_license_file_content,
        render_mit_license_with_fillers(Some(USER_OPTION_VALUE), Some(YEAR_OPTION_VALUE))
    );
}

#[test]
#[serial]
fn licensit_add_with_year_option_and_env_variable() {
    let temp_dir_context = TempDirContext::new();
    let year_option_with_value = format!("--year={}", YEAR_OPTION_VALUE);
    create_licensit_add_command()
        .arg(MIT_LICENSE_NAME)
        .arg(year_option_with_value)
        .env(
            LICENSE_AUTHOR_ENV_VARIABLE_NAME,
            LICENSE_AUTHOR_ENV_VARIABLE_VALUE,
        )
        .assert()
        .success();
    let added_license_file_content =
        fs::read_to_string(temp_dir_context.path().join("LICENSE")).unwrap();
    assert_eq!(
        added_license_file_content,
        render_mit_license_with_fillers(
            Some(LICENSE_AUTHOR_ENV_VARIABLE_VALUE),
            Some(YEAR_OPTION_VALUE)
        )
    );
}

#[test]
#[serial]
fn licensit_add_with_all_options_and_env_variable() {
    let temp_dir_context = TempDirContext::new();
    let user_option_with_value = format!("--user={}", USER_OPTION_VALUE);
    let year_option_with_value = format!("--year={}", YEAR_OPTION_VALUE);
    create_licensit_add_command()
        .arg(MIT_LICENSE_NAME)
        .arg(user_option_with_value)
        .arg(year_option_with_value)
        .env(
            LICENSE_AUTHOR_ENV_VARIABLE_NAME,
            LICENSE_AUTHOR_ENV_VARIABLE_VALUE,
        )
        .assert()
        .success();
    let added_license_file_content =
        fs::read_to_string(temp_dir_context.path().join("LICENSE")).unwrap();
    assert_eq!(
        added_license_file_content,
        render_mit_license_with_fillers(Some(USER_OPTION_VALUE), Some(YEAR_OPTION_VALUE))
    );
}
