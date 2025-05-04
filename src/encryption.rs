use aes_gcm::{
    aead::{Aead, KeyInit, OsRng, rand_core::RngCore},
    Aes256Gcm, Nonce,
};
use tracing::info;
use std::path::Path;

use crate::{config::Config, error::SafeFileError, io, validation};

/// Шифрует файл «на месте» (формат: nonce + ciphertext).
pub fn encrypt_file(path: &Path) -> Result<(), SafeFileError> {
    validation::validate_path(path)?;
    let data = io::read_file(path)?;

    let cipher = Aes256Gcm::new_from_slice(&Config::global().encryption_key)
        .map_err(|e| SafeFileError::Crypto(e.to_string()))?;

    let mut nonce_bytes = [0u8; 12];
    OsRng.fill_bytes(&mut nonce_bytes);
    let nonce = Nonce::from_slice(&nonce_bytes);

    let mut ciphertext = cipher
        .encrypt(nonce, data.as_ref())
        .map_err(|e| SafeFileError::Crypto(e.to_string()))?;

    let mut out = Vec::with_capacity(12 + ciphertext.len());
    out.extend_from_slice(&nonce_bytes);
    out.append(&mut ciphertext);

    io::write_file(path, &out)?;
    info!(target: "safe_file::encrypt", file = %path.display(), "файл зашифрован");
    Ok(())
}

/// Расшифровывает файл «на месте» (nonce берётся из первых 12 байт).
pub fn decrypt_file(path: &Path) -> Result<(), SafeFileError> {
    validation::validate_path(path)?;
    let enc = io::read_file(path)?;
    if enc.len() < 12 {
        return Err(SafeFileError::Crypto("слишком короткие данные".into()));
    }

    let (nonce_bytes, ciphertext) = enc.split_at(12);
    let nonce = Nonce::from_slice(nonce_bytes);

    let cipher = Aes256Gcm::new_from_slice(&Config::global().encryption_key)
        .map_err(|e| SafeFileError::Crypto(e.to_string()))?;

    let plaintext = cipher
        .decrypt(nonce, ciphertext)
        .map_err(|e| SafeFileError::Crypto(e.to_string()))?;

    io::write_file(path, &plaintext)?;
    info!(target: "safe_file::decrypt", file = %path.display(), "файл расшифрован");
    Ok(())
}
