use std::path::Path;
use crate::error::SafeFileError;

/// Проверка пути к файлу на корректность и безопасность
pub fn validate_path(p: &Path) -> Result<(), SafeFileError> {
    if p.as_os_str().is_empty() {
        return Err(SafeFileError::Validation("пустой путь".into()));
    }
    if !p.is_file() {
        return Err(SafeFileError::Validation("это не файл".into()));
    }
    Ok(())
}