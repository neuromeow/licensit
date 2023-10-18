# licensit

`licensit` is a command line tool to create LICENSE files.

### Supported licenses

- GNU Affero General Public License v3.0
- Apache License 2.0
- GNU General Public License v3.0
- GNU Lesser General Public License v3.0
- MIT License
- Mozilla Public License 2.0
- The Unlicense

## Installation

### Manual installation from GitHub

Compiled binary versions of `licensit` are uploaded to GitHub when a release is made.
You can install `licensit` manually by [downloading a release](https://github.com/neuromeow/licensit/releases), extracting it, and copying the binary to a directory in your `$PATH`, such as `/usr/local/bin`.

### Cargo

If you already have a Rust environment set up, you can use the `cargo install` command:

```
cargo install licensit
```

Cargo will build the `licensit` binary and place it in `$HOME/.cargo`.

## Usage

```
Command line tool to create LICENSE files

Usage: licensit <COMMAND>

Commands:
  list  Print a list of all available licenses
  show  Print the content of the selected license
  add   Add the selected license to the current directory
  help  Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

```
Print a list of all available licenses

Usage: licensit list

Options:
  -h, --help  Print help
```

```
Print the content of the selected license

Usage: licensit show [OPTIONS] <LICENSE>

Arguments:
  <LICENSE>  Selected license [possible values: agpl-3.0, apache-2.0, gpl-3.0, lgpl-3.0, mit, mpl-2.0, unlicense]

Options:
  -u, --user <USER>  The user or organization who holds the license [default: neuromeow]
  -y, --year <YEAR>  The year the license is in effect [default: 2023]
  -t, --template     License template only, no fillers for user or organization and year
  -h, --help         Print help
```

```
Add the selected license to the current directory

Usage: licensit add [OPTIONS] <LICENSE>

Arguments:
  <LICENSE>  Selected license [possible values: agpl-3.0, apache-2.0, gpl-3.0, lgpl-3.0, mit, mpl-2.0, unlicense]

Options:
  -u, --user <USER>  The user or organization who holds the license [default: neuromeow]
  -y, --year <YEAR>  The year the license is in effect [default: 2023]
  -h, --help         Print help
```

```
Print this message or the help of the given subcommand(s)

Usage: licensit help [COMMAND]...

Arguments:
  [COMMAND]...  Print help for the subcommand(s)
```

## Limitations

If the `--year` option is omitted, the current year is used.

To determine the user or organization who holds the license, the following order is used:

- Command line option: `-u`, `--user`
- Environment variable: `LICENSE_AUTHOR`
- `user.name` variable in the `$HOME/.gitconfig` file
- Username associated with the current effective user ID

If your name is set in the `$HOME/.gitconfig` file, you can omit the `--user` option.

## License

This project is released under the MIT License.
See [LICENSE](https://github.com/neuromeow/licensit/blob/master/LICENSE) for the full licensing condition.
