<div align="center">
<h1>Dono</h1>
<img alt="GitHub Workflow Status" src="https://img.shields.io/github/actions/workflow/status/jervw/dono/rust.yml">
<img alt="GitHub" src="https://img.shields.io/github/license/jervw/dono">
<img alt="Crates.io" src="https://img.shields.io/crates/v/dono-cli">

<p>A customizable CLI tool to show your GitHub contributions graph in your terminal</p>
<img src=preview.png>

</div>

## Installation

### Cargo package

```
cargo install dono-cli
```

### Build from source
```sh
git clone https://github.com/jervw/dono
cd dono
cargo build --release
cargo run
```

## Generating a personal access token

Navigate to your GitHub [developer settings](https://github.com/settings/tokens). Click the Personal access tokens menu, then click Generate new token (classic).

Scopes are not required.

GitHub will display the personal access token once. Copy the token into `dono.toml` configuration file.


## Configuration

After installing `dono`, edit the [example configuration](https://raw.githubusercontent.com/jervw/dono/main/dono.toml) to your liking.
The location is `$XDG_CONFIG_HOME/dono/dono.toml`.

## Usage

### `dono --help`

```
A CLI tool to show your GitHub contributions

Usage: dono <user_name>

Arguments:
  <user_name>  GitHub user name

Options:
  -h, --help     Print help information
  -V, --version  Print version information
```

## Features/Bugs

Please create an issue :)

## TODOs
- [ ] Week starts on monday setting
- [ ] Display stats (e.g. streaks, highest contributions)
- [ ] Custom queries


## License

This project is licensed under the MIT. See the [LICENSE](LICENSE) file for details.
