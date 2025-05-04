use std::path::Path;
use backoff::{ExponentialBackoff, Operation};

use crate::{decrypt_file, encrypt_file, error::SafeFileError, verify_file};

/// Перезапуск операций с экспоненциальным бэк‑оффом
fn retry<O, F>(mut op: F) -> Result<(), SafeFileError>
where
    F: FnMut() -> Result<(), SafeFileError>,
{
    let mut backoff = ExponentialBackoff {
        max_elapsed_time: Some(std::time::Duration::from_secs(30)),
        ..Default::default()
    };

    Operation::new(&mut backoff).retry(|| op())
}

/// Безопасное шифрование с повторами
pub fn encrypt_with_retry(path: &Path) -> Result<(), SafeFileError> {
    retry(|| encrypt_file(path))
}

/// Безопасная верификация подписи с повторами
pub fn verify_with_retry(path: &Path) -> Result<(), SafeFileError> {
    retry(|| verify_file(path))
}

/// Безопасная дешифрация с повторами
pub fn decrypt_with_retry(path: &Path) -> Result<(), SafeFileError> {
    retry(|| decrypt_file(path))
}
