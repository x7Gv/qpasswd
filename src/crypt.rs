
use crypto::{aes, digest::Digest, blockmodes::PkcsPadding};
use crypto::symmetriccipher;
use crypto::buffer;
use crypto::sha2;

use crypto::buffer::*;

pub fn encrypt(data: &[u8], key: &[u8], iv: &mut [u8]) -> Result<Vec<u8>, symmetriccipher::SymmetricCipherError> {

    let mut hasher = sha2::Sha256::new();

    let mut hashed_key = [0u8; 32];

    hasher.input(key);
    hasher.result(&mut hashed_key);

    // println!("ec - {} - {} - {}", data.len(), key.len(), hashed_key.len());

    let mut encryptor = aes::cbc_encryptor(
        aes::KeySize::KeySize256,
        &hashed_key,
        iv,
        PkcsPadding
    );

    let mut final_res = Vec::<u8>::new();
    let mut buf = [0; 4096];
    let mut read_buf = buffer::RefReadBuffer::new(data);
    let mut write_buf = buffer::RefWriteBuffer::new(&mut buf);

    loop {
        let res = encryptor.encrypt(&mut read_buf, &mut write_buf, true)?;

        final_res.extend(write_buf.take_read_buffer().take_remaining().iter().map(|&i| i));

        match res {
            BufferResult::BufferUnderflow => break,
            BufferResult::BufferOverflow => {  }
        }
    }

    Ok(final_res)
}

pub fn decrypt(data: &[u8], key: &[u8], iv: &mut [u8]) -> Result<Vec<u8>, crypto::symmetriccipher::SymmetricCipherError> {

    let mut hasher = sha2::Sha256::new();
    let mut hashed_key = [0u8; 32];

    hasher.input(key);
    hasher.result(&mut hashed_key);

    // println!("dc - {} - {} - {}", data.len(), key.len(), hashed_key.len());

    let mut decryptor = aes::cbc_decryptor(
        aes::KeySize::KeySize256,
        &hashed_key,
        iv,
        PkcsPadding
    );

    let mut final_res = Vec::<u8>::new();

    let mut buf = [0; 4096];

    let mut read_buf = buffer::RefReadBuffer::new(data);
    let mut write_buf = buffer::RefWriteBuffer::new(&mut buf);

    loop {
        let res = decryptor.decrypt(&mut read_buf, &mut write_buf, true)?;

        final_res.extend(write_buf.take_read_buffer().take_remaining().iter().map(|&i| i));
        match res {
            BufferResult::BufferUnderflow => break,
            BufferResult::BufferOverflow => { }
        }
    }

    Ok(final_res)
}
