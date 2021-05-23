use crypto::buffer;
use crypto::sha2;
use crypto::symmetriccipher;
use crypto::{aes, blockmodes::PkcsPadding, digest::Digest};

use crypto::buffer::*;

pub fn derive_key(pass: &str, key: &mut [u8]) {
    let scrypt_p = crypto::scrypt::ScryptParams::new(8, 256, 4);
    crypto::scrypt::scrypt(pass.as_bytes(), b"salt", &scrypt_p, key);
}

pub fn encrypt(
    data: &[u8],
    key: &[u8],
    iv: &mut [u8],
) -> Result<Vec<u8>, symmetriccipher::SymmetricCipherError> {
    let mut hasher = sha2::Sha256::new();

    let mut hashed_key = [0u8; 32];

    hasher.input(key);
    hasher.result(&mut hashed_key);

    // println!("ec - {} - {} - {}", data.len(), key.len(), hashed_key.len());

    let mut encryptor = aes::cbc_encryptor(aes::KeySize::KeySize256, &hashed_key, iv, PkcsPadding);

    let mut final_res = Vec::<u8>::new();
    let mut buf = [0; 4096];
    let mut read_buf = buffer::RefReadBuffer::new(data);
    let mut write_buf = buffer::RefWriteBuffer::new(&mut buf);

    loop {
        let res = encryptor.encrypt(&mut read_buf, &mut write_buf, true)?;

        final_res.extend(
            write_buf
                .take_read_buffer()
                .take_remaining()
                .iter()
                .map(|&i| i),
        );

        match res {
            BufferResult::BufferUnderflow => break,
            BufferResult::BufferOverflow => {}
        }
    }

    Ok(final_res)
}

pub fn decrypt(
    data: &[u8],
    key: &[u8],
    iv: &mut [u8],
) -> Result<Vec<u8>, crypto::symmetriccipher::SymmetricCipherError> {
    let mut hasher = sha2::Sha256::new();
    let mut hashed_key = [0u8; 32];

    hasher.input(key);
    hasher.result(&mut hashed_key);

    // println!("dc - {} - {} - {}", data.len(), key.len(), hashed_key.len());

    let mut decryptor = aes::cbc_decryptor(aes::KeySize::KeySize256, &hashed_key, iv, PkcsPadding);

    let mut final_res = Vec::<u8>::new();

    let mut buf = [0; 4096];

    let mut read_buf = buffer::RefReadBuffer::new(data);
    let mut write_buf = buffer::RefWriteBuffer::new(&mut buf);

    loop {
        let res = decryptor.decrypt(&mut read_buf, &mut write_buf, true)?;

        final_res.extend(
            write_buf
                .take_read_buffer()
                .take_remaining()
                .iter()
                .map(|&i| i),
        );
        match res {
            BufferResult::BufferUnderflow => break,
            BufferResult::BufferOverflow => {}
        }
    }

    Ok(final_res)
}

mod tests {

    #[test]
    fn enc_dec() {
        use rand::RngCore;
        use rand_core::OsRng;

        let msg = "test";
        let mut key: [u8; 32] = [0; 32];
        let mut iv: [u8; 16] = [0; 16];

        OsRng.fill_bytes(&mut iv);
        OsRng.fill_bytes(&mut key);

        let enc_data = super::encrypt(msg.as_bytes(), &key, &mut iv).unwrap();
        let dec_data = super::decrypt(&enc_data[..], &key, &mut iv).unwrap();

        assert!(msg.as_bytes() == &dec_data[..]);
    }

    /*
     *  TOO RESOURCE INTENSIVE
     *
    #[test]
    fn key_derivation() {

        let mut key0 = [0u8; 32];
        let mut key1 = [0u8; 32];
        let mut key2 = [0u8; 32];

        super::derive_key("jea", &mut key0);
        super::derive_key("jea", &mut key1);
        super::derive_key("jou", &mut key2);

        assert!(key0 == key1);
        assert!(key0 != key2);
    }
    */
}
