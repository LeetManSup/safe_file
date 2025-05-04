//! Функции‑обёртки, выполняющие операции с повтором по экспоненциальному back‑off.

use std::{path::Path, thread::sleep};

use backoff::{backoff::Backoff, ExponentialBackoff};
use crate::{
    decrypt_file, encrypt_file, verify_file,
    error::SafeFileError,
};

/// Универсальный ретрай — повторяет `op` пока `Backoff` выдаёт задержки
fn retry_with_backoff<F>(mut op: F) -> Result<(), SafeFileError>
where
    F: FnMut() -> Result<(), SafeFileError>,
{
    let mut backoff = ExponentialBackoff::default();

    loop {
        match op() {
            Ok(v) => return Ok(v),   // успех: выходим
            Err(e) => {
                // если Backoff вернул None — лимит исчерпан, отдаём ошибку наружу
                if let Some(delay) = backoff.next_backoff() {
                    sleep(delay);
                } else {
                    return Err(e);
                }
            }
        }
    }
}

/// Повторить шифрование файла до успеха или исчерпания back‑off.
pub fn encrypt_with_retry(path: &Path) -> Result<(), SafeFileError> {
    retry_with_backoff(|| encrypt_file(path))
}

/// Повторить проверку подписи до успеха или исчерпания back‑off.
pub fn verify_with_retry(path: &Path) -> Result<(), SafeFileError> {
    retry_with_backoff(|| verify_file(path))
}

/// Повторить дешифрацию файла до успеха или исчерпания back‑off.
pub fn decrypt_with_retry(path: &Path) -> Result<(), SafeFileError> {
    retry_with_backoff(|| decrypt_file(path))
}
