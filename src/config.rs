use once_cell::sync::OnceCell;
use rand_core::{OsRng, RngCore};
use ed25519_dalek::SigningKey;

/// Глобальная конфигурация: ключ шифрования и секретный ключ подписи.
#[derive(Debug)]
pub struct Config {
    /// 256‑битный ключ, используемый модулем AES‑GCM.
    pub encryption_key: [u8; 32],
    /// Секретный ключ Ed25519 для подписи файлов.
    pub signing_key:    SigningKey,
}

static CONFIG: OnceCell<Config> = OnceCell::new();

impl Config {
    /// Инициализировать глобальную конфигурацию (один раз за процесс).
    pub fn init(encryption_key: [u8; 32], signing_key: SigningKey) {
        CONFIG
            .set(Config { encryption_key, signing_key })
            .expect("Config already initialized");
    }

    /// Получить ссылку на конфигурацию; паника, если не инициализирована.
    pub fn global() -> &'static Config {
        CONFIG.get().expect("Config not initialized")
    }
}

/// Быстро сгенерировать пару ключей для тестов/демо.
pub fn generate_keys() -> ([u8; 32], SigningKey) {
    let mut key = [0u8; 32];
    OsRng.fill_bytes(&mut key);
    let sk = SigningKey::generate(&mut OsRng);
    (key, sk)
}
