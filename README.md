# qpasswd
A Symmetric key based password crypting tool.
 Using `AES-256-CBC` as an encryption standard and `scrypt` with 256bit keys as the key derivation for the CBC algorithm.

## Building
This project requires Rusts Cargo build system for it to be built. 
The project can be built with the following command.
```
$ cargo build --release
```

## Usage (core)
```
USAGE:
    qpasswd [FLAGS] <SUBCOMMAND>

FLAGS:
        --debug      Enable debug mode
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    crypt    Symmetric crypting tool
    gen      generates passwords
    help     Prints this message or the help of the given subcommand(s)
```
## Usage (crypt)
```
USAGE:
    qpasswd crypt [FLAGS] -d -e --pass <pass> --source <source>

FLAGS:
    -d               decrypt mode
    -e               encrypt mode
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -p, --pass <pass>        Passphrase to derive the key from.
    -s, --source <source>    Source to crypt
```
## Usage (gen)
```
USAGE:
    qpasswd gen [FLAGS] -l <length>

FLAGS:
    -h, --help         Prints help information
        --lowercase    Enable lowercase charset
        --numbers      Enable number charset
        --special      Enable special charset
        --symbols      Enable symbols charset
        --uppercase    Enable uppercase charset
    -V, --version      Prints version information

OPTIONS:
    -l <length>        The number of characters
```
