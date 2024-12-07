# Mini Password Manager

A simple password manager written in Rust.

- Add passwords
- List passwords
- Search passwords
- Delete passwords

## Usage

```
$ ./target/debug/gestion-mdp --help
Password Manager 1.0
B Mathieu <mathieu.busse24@gmail.com>

USAGE:
    gestion-mdp [OPTIONS] <SUBCOMMAND>

OPTIONS:
    -h, --help    Print help information
    -V, --version Print version information

SUBCOMMANDS:
    add    Add a new password
    delete Delete a password
    list   List all passwords
    search Search a password
```

## Installation

### From source

```bash
$ git clone https://github.com/mathieu-busse/gestion-mdp.git
$ cd gestion-mdp
$ cargo build --release
$ ./target/release/gestion-mdp --help
```

