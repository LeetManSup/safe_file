//! Публичное API библиотеки safe_file.

#![forbid(unsafe_code)]
#![warn(missing_docs)]

/// Глобальная конфигурация, генерация ключей.
pub mod config;
/// Модуль симметричного шифрования и дешифрования файлов.
pub mod encryption;
/// Типы ошибок, возвращаемых библиотекой.
pub mod error;
/// Механизмы повторных попыток (retry) при сбоях.
pub mod fallback;
/// Безопасное чтение и атомарная запись файлов.
pub mod io;
/// Инициализация логирования через `tracing`.
pub mod logger;
/// ЭЦП: создание и проверка подписи файлов.
pub mod signature;
/// Валидация входных данных (пути, размеры и т.п.).
pub mod validation;
/// Упрощённый реэкспорт часто используемых элементов.
pub mod prelude;

pub use encryption::{decrypt_file, encrypt_file};
pub use signature::{sign_file, verify_file};

#[cfg(test)]
mod tests;