use thiserror::Error;

/// Общее перечисление ошибок библиотеки.
#[derive(Debug, Error)]
pub enum SafeFileError {
    #[error("ввод/вывод: {0}")]
    Io(#[from] std::io::Error),

    #[error("шифрование: {0}")]
    Crypto(String),

    #[error("подпись: {0}")]
    Signature(String),

    #[error("валидация входных данных: {0}")]
    Validation(String),

    #[error("конфигурация: {0}")]
    Config(String),
}
