pub mod crypt;

extern crate base64;

use std::convert::TryInto;

use structopt::StructOpt;

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

    /// Activate debug mode
    // long flags (--debug)
    #[structopt(long)]
    debug: bool,

    /// Insert source str
    #[structopt(name = "source", long, short, required_if("decrypt", "encrypt"))]
    source: String,

    /// Insert pass
    #[structopt(name = "pass", long, short, required_if("decrypt", "encrypt"))]
    pass: String,
}

fn run_encrypt(_data: &str, pass: &str, dbg: bool) {

    /*
     * Derive the symmetric encryption key from scrypt [RFC 7914]
     */
    let scrypt_p = crypto::scrypt::ScryptParams::new(8, 256, 4);
    let mut key = [0u8; 32];
    crypto::scrypt::scrypt(pass.as_bytes(), b"salt", &scrypt_p, &mut key);

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

    println!(">>>| {}", base64::encode(&res));
}

fn run_decrypt(data: &str, pass: &str, dbg: bool) {

    /*
     * Derive the symmetric encryption key from scrypt [RFC 7914]
     */
    let scrypt_p = crypto::scrypt::ScryptParams::new(8, 256, 4);
    let mut key = [0u8; 32];
    crypto::scrypt::scrypt(pass.as_bytes(), b"salt", &scrypt_p, &mut key);

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

    println!(">>>| {}", String::from_utf8_lossy(&res));
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let mut opt = Opt::from_args();

    let decrypt: bool = opt.decrypt;
    let encrypt: bool = opt.encrypt;
    let dbg: bool = opt.debug;
    let source: &str = &opt.source;
    let pass: &str = &opt.pass;

    if !(decrypt ^ encrypt) {
        println!("Please only use one of the possible flags. // !assert(decrypt ^ encrypt)");
        return Ok(())
    }
   
    if encrypt {
        println!("Attempting to encrypt :: This may take a while.");

        tokio::task::block_in_place(move || {
            run_encrypt(source, pass, dbg);
        });

    } else if decrypt {
        println!("Attempting to decrypt :: This may take a while.");

        tokio::task::block_in_place(move || {
            run_decrypt(source, &pass, dbg);
        });
    }

    if dbg {
        println!("{:?}", &mut opt);
    }

    Ok(())
}
