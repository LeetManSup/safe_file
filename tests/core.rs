//! tests/core.rs - проверяет публичный API safe_file
use std::{fs, io::Write};
use tempfile::{NamedTempFile, TempPath};

use safe_file::prelude::*;

/// Однократно инициализируем ключи для всех тестов
fn init_keys_once() {
    static INIT: std::sync::Once = std::sync::Once::new();
    INIT.call_once(|| {
        let (k, sk) = generate_keys();
        Config::init(k, sk);
    });
}

fn create_temp_file() -> (TempPath, Vec<u8>) {
    let mut tmp = NamedTempFile::new().expect("tmp file");
    let data = b"The quick brown fox jumps over the lazy dog".to_vec();
    tmp.write_all(&data).unwrap();
    (tmp.into_temp_path(), data)
}

/* ---------- базовые сценарии ---------- */

#[test]
fn encrypt_decrypt_roundtrip() {
    init_keys_once();
    let (path, original) = create_temp_file();
    encrypt_file(&path).unwrap();
    decrypt_file(&path).unwrap();
    assert_eq!(fs::read(&path).unwrap(), original);
}

#[test]
fn sign_and_verify_ok() {
    init_keys_once();
    let (path, _) = create_temp_file();
    sign_file(&path).unwrap();
    verify_file(&path).unwrap();
}

/* ---------- отрицательные сценарии ---------- */

#[test]
fn verify_fails_if_signature_missing() {
    init_keys_once();
    let (path, _) = create_temp_file();
    let err = verify_file(&path).unwrap_err();
    matches!(err, SafeFileError::Signature(_));
}

#[test]
fn verify_fails_on_tamper() {
    init_keys_once();
    let (path, _) = create_temp_file();
    sign_file(&path).unwrap();
    fs::write(&path, b"tampered").unwrap();
    let err = verify_file(&path).unwrap_err();
    matches!(err, SafeFileError::Signature(_));
}

/* ---------- надёжность ---------- */

#[test]
fn encrypt_with_retry_ok() {
    init_keys_once();
    let (path, _) = create_temp_file();
    safe_file::fallback::encrypt_with_retry(&path).unwrap();
    assert!(fs::metadata(&path).unwrap().len() > 12);
}
