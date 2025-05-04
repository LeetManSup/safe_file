use std::path::Path;
use crate::error::SafeFileError;

/// Убедиться, что путь ссылается на существующий файл.
pub fn validate_path(p: &Path) -> Result<(), SafeFileError> {
    if p.as_os_str().is_empty() {
        return Err(SafeFileError::Validation("пустой путь".into()));
    }
    if !p.is_file() {
        return Err(SafeFileError::Validation("это не файл".into()));
    }
    Ok(())
}