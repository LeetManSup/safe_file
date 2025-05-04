use std::{
    fs::File,
    io::{Read, Write},
    path::Path,
};

use tempfile::NamedTempFile;

use crate::{error::SafeFileError, validation::validate_path};

const BUFFER_SIZE: usize = 8 * 1024; // 8 КиБ

/// Прочитать файл в вектор байт (буферами по 8 КиБ)
pub fn read_file(path: &Path) -> Result<Vec<u8>, SafeFileError> {
    validate_path(path)?;
    let mut file = File::open(path)?;
    let mut buf = Vec::new();
    let mut chunk = [0u8; BUFFER_SIZE];
    loop {
        let n = file.read(&mut chunk)?;
        if n == 0 {
            break;
        }
        buf.extend_from_slice(&chunk[..n]);
    }
    Ok(buf)
}

/// Атомарно записать данные в файл (tmp‑файл + rename)
pub fn write_file(path: &Path, data: &[u8]) -> Result<(), SafeFileError> {
    let dir = path.parent()
        .ok_or_else(|| SafeFileError::Validation("некорректный путь".into()))?;

    let mut tmp = NamedTempFile::new_in(dir)?;
    tmp.write_all(data)?;
    tmp.flush()?;

    // persist возвращает PersistError; конвертируем в std::io::Error
    tmp.persist(path)
        .map_err(|e| SafeFileError::Io(e.error))?;
    Ok(())
}
