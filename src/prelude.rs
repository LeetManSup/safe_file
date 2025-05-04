//! Для быстрого импорта.

pub use crate::error::SafeFileError;
pub use crate::config::{Config, generate_keys};
pub use crate::logger::init as init_logger;
pub use crate::{encrypt_file, decrypt_file, sign_file, verify_file};
