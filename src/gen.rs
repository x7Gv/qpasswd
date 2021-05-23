use anyhow::Result;
use rand::seq::SliceRandom;
use rand_core::OsRng;

#[derive(Debug)]
pub enum CharsetType {
    Lowercase,
    Uppercase,
    Symbols,
    Numbers,
    Special,
}

#[derive(Debug, Default)]
pub struct PasswdGenBuilder {
    length: i16,
    charsets: Vec<CharsetType>,
}

#[derive(Debug, Default)]
pub struct PasswdGen {
    length: i16,
    charsets: Vec<CharsetType>,
}

fn charset(charset: &CharsetType) -> Vec<char> {
    match charset {
        CharsetType::Lowercase => {
            "abcdefghijklmnopqrstuvwxyz".chars().collect()
        },
        CharsetType::Uppercase => {
            "ABCDEFGHIJKLMNQRSTUVWXYZ".chars().collect()
        },
        CharsetType::Symbols => {
            "_*&|!?@$#=%".chars().collect()
        },
        CharsetType::Numbers => {
            "0123456789".chars().collect()
        }
        CharsetType::Special => {
            r###"!"#$%&'()*+,-./:;<=>?@[\]^_`{|}~"###.chars().collect()
        }
    }
}

impl PasswdGenBuilder {

    pub fn add_charset(&mut self, charset: CharsetType) -> &mut Self {
        self.charsets.push(charset);
        self
    }

    pub fn set_length(&mut self, length: i16) -> &mut Self {
        self.length = length;
        self
    }

    pub fn build(self) -> PasswdGen {
        PasswdGen {
            length: self.length,
            charsets: self.charsets,
        }
    }
}

impl PasswdGen {

    pub fn builder() -> PasswdGenBuilder {
        PasswdGenBuilder::default()
    }

    pub fn generate(&mut self) -> Result<String> {

        let mut s = String::new();
        let mut c = Vec::<char>::new();

        for chset in &self.charsets {
            c.append(&mut charset(chset));
        }

        for _ in 0..self.length {
            s.push(*c.choose(&mut rand::thread_rng()).unwrap());
        }

        Ok(s)
    }
}
