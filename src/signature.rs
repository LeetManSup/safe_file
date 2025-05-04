use std::{fs, path::Path};

use ed25519_dalek::{Signature, VerifyingKey, Signer, Verifier};
use tracing::info;

use crate::{config::Config, error::SafeFileError, validation};

const SIG_EXT: &str = ".sig";

/// Создать подпись файла и сохранить её рядом (`file.ext.sig`).
pub fn sign_file(path: &Path) -> Result<(), SafeFileError> {
    validation::validate_path(path)?;
    let data = fs::read(path)?;

    let sk = &Config::global().signing_key;
    let sig: Signature = sk.sign(&data);

    let sig_path = path.with_extension(format!(
        "{}{}",
        path.extension().unwrap_or_default().to_string_lossy(),
        SIG_EXT
    ));

    fs::write(&sig_path, sig.to_bytes())?;
    info!(target: "safe_file::sign", file = %path.display(), "файл подписан");
    Ok(())
}

/// Проверить подпись файла; возвращает ошибку, если подпись отсутствует или неверна.
pub fn verify_file(path: &Path) -> Result<(), SafeFileError> {
    validation::validate_path(path)?;
    let data = fs::read(path)?;

    let sig_path = path.with_extension(format!(
        "{}{}",
        path.extension().unwrap_or_default().to_string_lossy(),
        SIG_EXT
    ));

    let sig_bytes = fs::read(&sig_path)
        .map_err(|_| SafeFileError::Signature("подпись не найдена".into()))?;

    let sig_arr: [u8; 64] = sig_bytes
        .try_into()
        .map_err(|_| SafeFileError::Signature("неверная длина подписи".into()))?;

    let sig = Signature::from_bytes(&sig_arr);

    let vk: VerifyingKey = Config::global().signing_key.verifying_key();

    vk.verify(&data, &sig)
        .map_err(|_| SafeFileError::Signature("проверка не пройдена".into()))?;

    info!(target: "safe_file::verify", file = %path.display(), "подпись верна");
    Ok(())
}
