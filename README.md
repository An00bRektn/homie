# homie

<p align="center">
    <img src = ./images/homie.jpg>
</p>

## Who is this?
`homie` is a command line tool that helps you keep track of hosts and directories (in the way I want to) while doing a security assessment of a network.

- 📋 **Intuitive flags** to update a list of targets at any time
- 📁 **Sets up folders on the fly** so you can get right to hacking instead of having to organize your things
- 🔥 **Written in Rust** because why not

**Contributions welcome!** This is my first Rust project that I've actually gone through with and I know I could be writing much more elegant code. If this is helpful and there are changes you want to see, drop a PR! :)

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

## Usage
```
PS > homie.exe -h
a homie that's really into organization

Usage: homie.exe [OPTIONS] [COMMAND]

Commands:
  add     Adds a new host to the config file
  delete  Deletes a host from the config file
  info    Gets information on the hosts stored in the config file (can specify a host by IP)
  init    Initializes a new workspace
  update  Updates an existing entry in .homie.yml
  help    Print this message or the help of the given subcommand(s)

Options:
  -c, --config <FILE>  Specifies path to config file (default searches parent directories for .homie.yml)
  -h, --help           Print help
  -V, --version        Print version
```

### Examples
#### Initialize Workspace
```
PS > homie init
[*] Current Directory: "C:\\Users\\homie\\examplecorp.local"
[?] Would you like to initialize a homie workspace in the current directory? (y/n) y
[+] New workspace created at "C:\\Users\\homie\\examplecorp.local"
PS > cat .\.homie.yml
hosts:
```

#### Add and Update a Host
```
PS > homie add -i 10.42.0.69 -n EXAMPLE-DC01 -d examplecorp.local -o windows -a false
[+] Adding IP Address: 10.42.0.69
[?] Did you want to create a new directory for this host [10.42.0.69]? (y/n) y
[*] Making workspace...
[+] Created! Located at "C:\\Users\\homie\\examplecorp.local\\10.42.0.69"
PS > cat .\.homie.yml
hosts:
  10.42.0.69:
    hostname: EXAMPLE-DC01
    os: windows
    access: false
    domain: examplecorp.local
PS > homie update -i 10.42.0.69 -a true                                              
[*] Updating...
[+] Updated! New info for 10.42.0.69:
Host {
    hostname: "EXAMPLE-DC01",
    os: "windows",
    access: true,
    domain: Some(
        "examplecorp.local",
    ),
}
```

#### Delete a Host
```
PS > homie.exe delete -i 10.42.0.69        
[?] Are you sure you want to delete the entry for 10.42.0.69? (y/n) y
[?] Did you want to delete the directory for 10.42.0.69? (y/n) y
[*] Deleting workspace...
[+] Done!
```

## FAQ
### Couldn't you just do this with a shell script?
Yes, but Rust.

### Why did you implement X so weirdly when the Y function exists in Rust?
I'm still very new to Rust and know there's a lot of things I could have done instead but am still working my way through understanding it.

### Why YAML?
Simple enough to modify by hand if you don't feel like using the command line tool, and the [serde-yaml](https://github.com/dtolnay/serde-yaml) crate is pretty solid.

### Why do you look like that?
Rude.

## TODO
- [ ] Implement a local store (sqlite) for credentials
    - [ ] Ability to add user, password, and note
    - [ ] Ability to update password keyed by username
    - [ ] Ability to delete based on username
- [ ] Pretty print with colors like jq
- [ ] Verbosity flag?
- [ ] Refactoring!
  - [ ] More error handling and checking
  - [ ] Make code cleaner (I'm not good at Rust)
  - [ ] Redesign for arbitrary configuration to allow this to be used for more than just security assessments (would really only need a single config)