# homie

<p align="center">
    <img src = ./images/homie.jpg>
</p>

## Who is this?
`homie` is a command line tool that helps you keep track of hosts and directories (in the way I want to) while doing a security assessment of a network.

- 📋 **Intuitive flags** to update a list of targets at any time
- 📁 **Sets up folders** so you can get right to hacking instead of organizing
- 🔥 **Written in Rust** because why not

## Usage
```
PS > homie.exe -h
a homie that's really into organization

Usage: homie.exe [OPTIONS] [COMMAND]

Commands:
  add
  info
  help  Print this message or the help of the given subcommand(s)

Options:
  -c, --config <FILE>  Specifies path to config file (default searches parent directories for .homie.yml)
  -h, --help           Print help
  -V, --version        Print version
```

## Install
### Precompiled Binaries
Precompiled binaries (Windows/Linux/macOS) are available at [releases](https://github.com/nicocha30/An00bRektn/homie/releases).

### Compile From Source
Prereqs: [Rust](https://rustup.rs/)
```
git clone https://github.com/An00bRektn/homie.git
cd homie
cargo build --release
```
The executable will be located at `target/release/homie`.

## TODO
- [ ] Add the ability to update a field for an existing host
- [ ] Add the ability to delete hosts from `.homie.yml`
- [ ] Implement a local store (sqlite) for credentials
    - [ ] Ability to add user, password, and note
    - [ ] Ability to update password keyed by username
    - [ ] Ability to delete based on username
- [ ] Pretty print with colors like jq
- [ ] More error handling and checking