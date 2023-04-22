use std::path::{PathBuf, MAIN_SEPARATOR};

use crate::consts::VAULT_FILE_EXTENSION;
use crate::vault::{header, key_derivation::KeyDerivationFunc};
use color_eyre::{eyre::eyre, Report, Result};

pub fn run(path: &str) -> Result<(), Report> {
    if path.ends_with(MAIN_SEPARATOR) {
        return Err(eyre!("Vault name must be a file path"));
    }
    let mut filename_path = PathBuf::from(path);
    filename_path.set_extension(VAULT_FILE_EXTENSION);

    // TODO: make this configurable
    let kdf = KeyDerivationFunc::Argon2;
    let header = header::HeaderBuilder::default().kdf(kdf).build()?;
    let mut header_text = String::new();
    dbg!(header.to_string(&mut header_text)?);

    Ok(())
}
