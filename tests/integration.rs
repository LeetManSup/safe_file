use safe_file::*;
use std::io::Write;
use tempfile::NamedTempFile;

#[test]
fn roundtrip() {
    // инициализация
    let (key, sk) = config::generate_keys();
    config::Config::init(key, sk);
    logger::init();

    // создаём файл и пишем данные
    let mut tmp = NamedTempFile::new().unwrap();
    tmp.write_all(b"integration").unwrap();

    // закрываем дескриптор и получаем путь
    let path = tmp.into_temp_path();   // важная строка!
    let path = path.to_path_buf();     // TempPath -> PathBuf

    // операции библиотеки
    encrypt_file(&path).unwrap();
    decrypt_file(&path).unwrap();
    sign_file(&path).unwrap();
    verify_file(&path).unwrap();
}
