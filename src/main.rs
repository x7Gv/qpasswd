pub mod crypt;
pub mod gen;

extern crate base64;

use std::convert::TryInto;
use std::io::BufRead;
use std::time::Instant;

use clap::{App, Arg, AppSettings};

fn run_encrypt(_data: &str, pass: &str, dbg: bool) {

    /*
     * Derive the symmetric encryption key from scrypt [RFC 7914]
     */
    let mut key = [0u8; 32];
    crypt::derive_key(pass, &mut key);

    qpasswd::random_art(&key);

    if dbg {
        println!("pass -> {:?}", &pass);
        println!("key -> {:?}", key);
        println!("size -> {}", key.len());
    }

    // Derive the IV from the first block of derived key.
    let mut iv: [u8; 16] = key[0..16].try_into().unwrap();

    if dbg {
        println!("{:?}", iv);
    }

    let res = crypt::encrypt(_data.as_bytes(), &key, &mut iv).unwrap();

    println!("+------------------------------+");
    println!(">>>| {}", base64::encode(&res));
    println!("+------------------------------+");
}

fn run_decrypt(data: &str, pass: &str, dbg: bool) {

    /*
     * Derive the symmetric encryption key from scrypt [RFC 7914]
     */
    let mut key = [0u8; 32];
    crypt::derive_key(pass, &mut key);

    qpasswd::random_art(&key);

    if dbg {
        println!("pass -> {:?}", &pass);
        println!("key -> {:?}", key);
        println!("size -> {}", key.len());
    }

    // Derive the IV from the first block of derived key.
    let mut iv: [u8; 16] = key[0..16].try_into().unwrap();

    if dbg {
     println!("{:?}", iv);
    }

    let res = crypt::decrypt(&base64::decode(&data).unwrap(), &key, &mut iv).unwrap();

    println!("+------------------------------+");
    println!(">>>| {}", String::from_utf8_lossy(&res));
    println!("+------------------------------+");
}

fn ask_for_copy() {

    println!("copy y/n");

    loop {
        match std::io::stdin().lock().lines().next().unwrap().unwrap().to_string().as_str() {
            "y" => {
                println!("yes");
                break;
            },
            "n" => {
                println!("no");
                break;
            },
            _ => {
                println!("none");
                break;
            }
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let matches = App::new("qpasswd")
        .version("0.5.0")
        .author("osk")
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .arg(
            Arg::with_name("debug")
                .long("debug")
                .help("Enable debug mode")
        )
        .subcommand(
            App::new("gen")
                .about("generates passwords")
                .arg(
                    Arg::with_name("lowercase")
                        .long("lowercase")
                        .help("Enable lowercase charset")
                )
                .arg(
                    Arg::with_name("uppercase")
                        .long("uppercase")
                        .help("Enable uppercase charset")
                )
                .arg(
                    Arg::with_name("symbols")
                        .long("symbols")
                        .help("Enable symbols charset")
                )
                .arg(
                    Arg::with_name("numbers")
                        .long("numbers")
                        .help("Enable number charset")
                )
                .arg(
                    Arg::with_name("special")
                        .long("special")
                        .help("Enable special charset")
                )
                .arg(
                    Arg::with_name("length")
                        .short("l")
                        .takes_value(true)
                        .required(true)
                        .help("The number of characters")
                ),
        )
        .subcommand(
            App::new("crypt")
                .about("Symmetric crypting tool")
                .arg(
                    Arg::with_name("encrypt")
                        .short("e")
                        .conflicts_with("decrypt")
                        .required_unless("encrypt")
                        .help("encrypt mode")
                )
                .arg(
                    Arg::with_name("decrypt")
                        .short("d")
                        .conflicts_with("encrypt")
                        .required_unless("decrypt")
                        .help("decrypt mode")
                )
                .arg(
                    Arg::with_name("pass")
                        .short("p")
                        .long("pass")
                        .takes_value(true)
                        .required(true)
                        .help("Passphrase to derive the key from.")
                )
                .arg(
                    Arg::with_name("source")
                        .short("s")
                        .long("source")
                        .takes_value(true)
                        .required(true)
                        .help("Source to crypt")
                )
        )
        .get_matches();

    let dbg = matches.is_present("debug");

    match matches.subcommand_name() {
        Some("gen") => {

            if let Some(args) = matches.subcommand_matches("gen") {
                if let Some(length) = args.value_of("length") {

                    let len = length.parse::<i16>().unwrap();

                    let mut gen = gen::PasswdGen::builder();
                    gen.set_length(len);

                    let mut configured = false;

                    if args.is_present("lowercase") {
                        gen.add_charset(gen::CharsetType::Lowercase);
                        configured = true;
                    }
                    if args.is_present("uppercase") {
                        gen.add_charset(gen::CharsetType::Uppercase);
                        configured = true;
                    }
                    if args.is_present("symbols") {
                        gen.add_charset(gen::CharsetType::Symbols);
                        configured = true;
                    }
                    if args.is_present("numbers") {
                        gen.add_charset(gen::CharsetType::Numbers);
                    }
                    if args.is_present("special") {
                        gen.add_charset(gen::CharsetType::Special);
                    }

                    let s: String;
                    if !configured {
                        gen
                            .add_charset(gen::CharsetType::Lowercase)
                            .add_charset(gen::CharsetType::Uppercase)
                            .add_charset(gen::CharsetType::Numbers);

                        let mut n: usize = 0;
                        for charset in &gen.charsets {
                            n = n + gen::charset(charset).len();
                        }

                        println!("Charsets enabled: {:?}", gen.charsets);
                        println!("Length: {}", gen.length);
                        println!("Entropy: {:.1} bits", (n as f64).powi(gen.length.into()).log(2.0));

                        s = gen.build().generate().unwrap();

                    } else {

                        let mut n: usize = 0;
                        for charset in &gen.charsets {
                            n = n + gen::charset(charset).len();
                        }

                        println!("Charsets enabled: {:?}", gen.charsets);
                        println!("Length: {}", gen.length);
                        println!("Entropy: {:.1} bits", (n as f64).powi(gen.length.into()).log(2.0));

                        s = gen.build().generate().unwrap();
                    }

                    println!("+------------------------------+");
                    println!(">>>| {}", s);
                    println!("+------------------------------+");

                    ask_for_copy();

                    return Ok(());
                }
            }
        },
        Some("crypt") => {

            if let Some(args) = matches.subcommand_matches("crypt") {

                let pass = args.value_of("pass").unwrap();
                let source = args.value_of("source").unwrap();

                if args.is_present("encrypt") {
                    println!("Attempting to encrypt :: This may take a while.");

                    tokio::task::block_in_place(move || {
                        let now = Instant::now();
                        run_encrypt(source, pass, dbg);
                        let elapsed = now.elapsed();

                        println!("elapsed : {:?}", elapsed);
                    });

                    return Ok(());
                }
                if args.is_present("decrypt") {

                    let source = args.value_of("source").unwrap();
                    let pass = args.value_of("pass").unwrap();

                    println!("Attempting to decrypt :: This may take a while.");

                    tokio::task::block_in_place(move || {

                        let now = Instant::now();
                        run_decrypt(source, &pass, dbg);
                        let elapsed = now.elapsed();

                        println!("elapsed : {:?}", elapsed);
                    });

                    return Ok(());
                }
            }
        },
        None => {
            println!("Subcommand not found")
        },
        _ => unreachable!()
    }

    Ok(())
}
