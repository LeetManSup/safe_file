#[cfg(test)]
mod tests {
    use tempfile::NamedTempFile;
    use std::io::Write;

    #[test]
    fn roundtrip_unit() {
        let (key, sk) = crate::config::generate_keys();
        crate::config::Config::init(key, sk);
        crate::logger::init();

        // 1. создаём и пишем
        let mut tmp = NamedTempFile::new().unwrap();
        tmp.write_all(b"hello").unwrap();

        // 2. закрываем дескриптор, получаем путь
        let path = tmp.into_temp_path();   // tmp автоматически закрывается здесь
        let path = path.to_path_buf();     // TempPath -> PathBuf

        // 3. операции с файлом
        crate::encrypt_file(&path).unwrap();
        crate::decrypt_file(&path).unwrap();
        crate::sign_file(&path).unwrap();
        crate::verify_file(&path).unwrap();
    }
}
