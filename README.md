# licensit

`licensit` is a command-line tool to create LICENSE files.

### Supported licenses

- GNU Affero General Public License v3.0 (AGPL-3.0)
- Apache License 2.0 (Apache-2.0)
- BSD 2-Clause “Simplified” License (BSD-2-Clause)
- BSD 3-Clause “New” or “Revised” License (BSD-3-Clause)
- Boost Software License 1.0 (BSL-1.0)
- Creative Commons Zero v1.0 Universal (CC0-1.0)
- Eclipse Public License 2.0 (EPL-2.0)
- GNU General Public License v2.0 (GPL-2.0)
- GNU General Public License v3.0 (GPL-3.0)
- GNU Lesser General Public License v2.1 (LGPL-2.1)
- MIT License (MIT)
- Mozilla Public License 2.0 (MPL-2.0)
- The Unlicense (Unlicense)

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

`licensit` simplifies the process of creating and managing license files for your projects.

### Listing Available Licenses

```
licensit list
```

Shows all supported licenses.

### Showing License Content

To view the content of a specific license with the author and year filled in:

```
licensit show [LICENSE] [--user USER] [--year YEAR]
```

- `[LICENSE]`: The ID of the license you want to display (for example, `mit`, `apache-2.0`)
- `--user [USER]`: Specifies the license holder's name. If not provided, `licensit` will use the following sources in order to determine the user name:
  - `LICENSE_AUTHOR` environment variable
  - `user.name` entry in the `$HOME/.gitconfig` file
  - Username associated with the current effective user ID
- `--year [YEAR]`: Sets the year during which the license is effective. Defaults to the current year if not specified

To display just the template of a license (without any specific user or year information):

```
licensit show [LICENSE] --template
```

- `[LICENSE]`: The ID of the license whose template you want to display (for example, `mit`, `apache-2.0`)
- `--template`: Displays the license template with placeholders for the user and year. This option cannot be used with `--user` or `--year`

### Adding a License to Your Project

To add a license file to your current directory:

```
licensit add [LICENSE] [--user USER] [--year YEAR]
```

Creates a `LICENSE` file in the current directory with the specified details.

### Help and Information

```
licensit help [COMMAND]
```

Provides detailed help for a specific command (`list`, `show`, or `add`).

## Limitations

To add a new license to `licensit`, place the license template file in the `data/licenses` directory or a subfolder. 
Then, update the `licenses.yml` file, specifying the short name, full name of the license, placeholders for the author and year (if needed), and the path to the template file within the `data/licenses` structure. 
This setup allows for the easy expansion of licensit by adding new licenses without altering the existing codebase.

In the current version of `licensit`, the tests primarily focus on command execution, covering basic scenarios such as specifying the user via an option or determining the author through an environment variable. 
Consequently, the automated tests are limited and utilize the MIT license template as a representative example. 
These tests do not cover scenarios without explicitly specifying a user or author, potentially missing out on capturing the full range of tool usage. 

## License

This project is released under the MIT License.
See [LICENSE](https://github.com/neuromeow/licensit/blob/master/LICENSE) for the full licensing condition.
