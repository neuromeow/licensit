use assert_cmd::Command;

#[test]
fn test_execute_licensit_list() {
    let mut cmd = Command::cargo_bin("licensit").unwrap();
    cmd.arg("list").assert().stdout(
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
