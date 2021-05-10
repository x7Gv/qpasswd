# qpasswd
A Symmetric key based password crypting tool.
 Using `AES-256-CBC` as an encryption standard and `scrypt` with 256bit keys as the key derivation for the CBC algorithm.

## Building
This project requires Rusts Cargo build system for it to be built. 
The project can be built with the following command.
```
$ cargo build --release
```

## Usage
```
USAGE:
    qpasswd [FLAGS] --pass <pass> --source <source>

FLAGS:
        --debug      Activate debug mode
    -d, --decrypt    Activate decrypt mode
    -e, --encrypt    Activate encrypt mode
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -p, --pass <pass>        Insert pass
    -s, --source <source>    Insert source str
```
