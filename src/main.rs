pub mod crypt;
pub mod gen;

extern crate base64;

use std::convert::TryInto;
use std::time::Instant;

use clap::{App, Arg, SubCommand, AppSettings};

use structopt::StructOpt;

#[derive(StructOpt)]
struct QPasswd {
    #[structopt(long)]
    debug: bool,

    #[structopt(subcommand)]
    cmd: Command,
}

#[derive(StructOpt)]
enum Command {
    Gen {
        #[structopt(long)]
        length: i16,

        #[structopt(long)]
        lowercase: bool,

        #[structopt(long)]
        uppercase: bool,

        #[structopt(long)]
        symbols: bool,

        #[structopt(long)]
        numbers: bool,

        #[structopt(long)]
        special: bool,
    },
    Crypt {
        /// Activate decrypt mode
        // short and long flags (-d, --decrypt)
        #[structopt(short, long)]
        decrypt: bool,

        /// Activate encrypt mode
        // short and long flags (-e, --encrypt)
        #[structopt(short, long)]
        encrypt: bool,

        /// Insert source str
        #[structopt(name = "source", long, short)]
        source: String,

        /// Insert pass
        #[structopt(name = "pass", long, short)]
        pass: String,
    }
}

/*
#[derive(Debug, StructOpt)]
#[structopt(name = "qpasswd")]
struct Opt {
    /// Activate decrypt mode
    // short and long flags (-d, --decrypt)
    #[structopt(short, long)]
    decrypt: bool,

    /// Activate encrypt mode
    // short and long flags (-e, --encrypt)
    #[structopt(short, long)]
    encrypt: bool,

    /// Activate gen mode
    // short and long flags (-g, --gen)
    #[structopt(short, long)]
    gen: bool,

    /// Toggle lowercase for gen
    #[structopt(name = "lower", long, short)]
    lower: bool,

    /// Toggle uppercase for gen
    #[structopt(name = "upper", long, short)]
    upper: bool,

    /// Toggle symbols for gen
    #[structopt(name = "symbols", long, short)]
    symbols: bool,

    /// Toggle numbers for gen
    #[structopt(name = "numbers", long, short)]
    numbers: bool,

    /// Toggle special for gen
    #[structopt(name = "special", long, short)]
    special: bool,

    /// Activate debug mode
    // long flags (--debug)
    #[structopt(long)]
    debug: bool,

    /// Insert source str
    #[structopt(name = "source", long, short)]
    source: String,

    /// Insert pass
    #[structopt(name = "pass", long, short)]
    pass: String,
}
*/

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

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let matches = App::new("qpasswd")
        .about("jea")
        .version("0.5.0")
        .author("osk")
        .arg(
            Arg::with_name("debug")
                .long("debug")
        )
        .subcommand(
            App::new("gen")
                .about("generates passwords")
                .arg(
                    Arg::with_name("lowercase")
                        .long("lowercase")
                )
                .arg(
                    Arg::with_name("uppercase")
                        .long("uppercase")
                )
                .arg(
                    Arg::with_name("symbols")
                        .long("symbols")
                )
                .arg(
                    Arg::with_name("length")
                        .short("l")
                        .takes_value(true)
                        .required(true)
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
                )
                .arg(
                    Arg::with_name("decrypt")
                        .short("d")
                        .conflicts_with("encrypt")
                        .required_unless("decrypt")
                )
                .arg(
                    Arg::with_name("pass")
                        .short("p")
                        .long("pass")
                        .takes_value(true)
                        .required(true)
                )
                .arg(
                    Arg::with_name("source")
                        .short("s")
                        .long("source")
                        .takes_value(true)
                        .required(true)
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

                    let s: String;
                    if !configured {
                        gen
                            .add_charset(gen::CharsetType::Lowercase)
                            .add_charset(gen::CharsetType::Uppercase)
                            .add_charset(gen::CharsetType::Numbers);

                        s = gen.build().generate().unwrap();

                    } else {
                        s = gen.build().generate().unwrap();
                    }

                    println!("+------------------------------+");
                    println!(">>>| {}", s);
                    println!("+------------------------------+");

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

    // let mut opt = Opt::from_args();

    /*
    let decrypt: bool = opt.decrypt;
    let encrypt: bool = opt.encrypt;
    let dbg: bool = opt.debug;
    let source: &str = &opt.source;
    let pass: &str = &opt.pass;

    let gen: bool = opt.gen;
    let lower: bool = opt.lower;
    let upper: bool = opt.upper;
    let symbols: bool = opt.symbols;
    let numbers: bool = opt.numbers;
    let special: bool = opt.special;
    */

    /*
    if gen {

        let mut builder = gen::PasswdGen::builder();

        if lower {
            builder.add_charset(gen::CharsetType::Lowercase);
        }
        if upper {
            builder.add_charset(gen::CharsetType::Uppercase);
        }
        if symbols {
            builder.add_charset(gen::CharsetType::Symbols);
        }
        if numbers {
            builder.add_charset(gen::CharsetType::Numbers);
        }
        if special {
            builder.add_charset(gen::CharsetType::Special);
        }

        builder.set_length(16);
        let s = builder.build().generate().unwrap();

        println!("+------------------------------+");
        println!(">>>| {}", s);
        println!("+------------------------------+");

        return Ok(())
    }

    if !(decrypt ^ encrypt) {
        println!("Please only use one of the possible flags. // !assert(decrypt ^ encrypt)");
        return Ok(())
    }
   
    if encrypt {
        println!("Attempting to encrypt :: This may take a while.");

        tokio::task::block_in_place(move || {

            let now = Instant::now();
            run_encrypt(source, pass, dbg);
            let elapsed = now.elapsed();

            println!("elapsed : {:?}", elapsed);
        });

    } else if decrypt {
        println!("Attempting to decrypt :: This may take a while.");

        tokio::task::block_in_place(move || {

            let now = Instant::now();
            run_decrypt(source, &pass, dbg);
            let elapsed = now.elapsed();

            println!("elapsed : {:?}", elapsed);
        });
    }

    if dbg {
        println!("{:?}", &mut opt);
    }
    */

    Ok(())
}
