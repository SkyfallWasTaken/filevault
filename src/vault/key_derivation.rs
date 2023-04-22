//! Utilities surrounding key derivation.

use argon2::Argon2;
use color_eyre::{eyre::eyre, Report, Result};
use rand::{rngs::StdRng, Rng};

pub type Key = [u8; 256];
pub const SALT_SLICE_LEN: usize = 256;
pub type Salt = [u8; SALT_SLICE_LEN];
pub const SALT_BASE64_LEN: usize = ((4 * SALT_SLICE_LEN / 3) + 3) & !3;

/// Key derivation function
#[derive(Default, Debug, Clone, Copy)]
pub enum KeyDerivationFunc {
    #[default]
    Argon2,
}

impl KeyDerivationFunc {
    pub fn gen_salt(rng: &mut StdRng) -> Salt {
        rng.gen()
    }

    pub fn gen_key(&self, password: String, salt: String) -> Result<Key, Report> {
        let password = password.as_bytes();
        let salt = salt.as_bytes();

        match self {
            KeyDerivationFunc::Argon2 => self.argon2(password, salt),
        }
    }

    fn argon2(&self, password: &[u8], salt: &[u8]) -> Result<Key, Report> {
        let mut output_key_material: Key = [0u8; 256];
        Argon2::default()
            .hash_password_into(password, salt, &mut output_key_material)
            .map_err(|_| eyre!("Failed to hash password into key"))?;
        Ok(output_key_material)
    }
}
