use std::str;

use super::key_derivation::{KeyDerivationFunc, SALT_BASE64_LEN};
use base64::{prelude::BASE64_STANDARD, Engine};
use color_eyre::Report;
use derive_builder::Builder;
use static_assertions::const_assert_eq;

#[allow(dead_code)]
pub const EXPECTED_IDENTIFIER_SIZE: usize = 10;
pub const IDENTIFIER: &str = "FILEVAULTx";

const_assert_eq!(IDENTIFIER.len(), EXPECTED_IDENTIFIER_SIZE);

#[derive(Default, Builder, Debug)]
#[builder(setter(into))]
pub struct Header {
    kdf: KeyDerivationFunc,
}

impl Header {
    pub fn to_string(&self, string: &mut String) -> Result<String, Report> {
        string.push_str(IDENTIFIER);
        // TODO: make this ALWAYS 3 characters
        string.push_str(&format!("{:03}", self.kdf as u8));

        // PANICS: RNG is always initialized at the start of the program
        let salt_slice = KeyDerivationFunc::gen_salt(&mut crate::THREAD_RNG.get().unwrap().clone());
        let salt_b64 = BASE64_STANDARD.encode(salt_slice);
        string.push_str(&salt_b64);

        Ok(string.clone())
    }
}
