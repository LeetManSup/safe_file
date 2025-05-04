use once_cell::sync::OnceCell;
use rand_core::OsRng;
use zeroize::Zeroize;

/// Параметры библиотеки (ключи, опции)
pub struct Config {
    /// Ключ 32 байта для AES-256-GCM
    pub encryption_key: [u8; 32],
    /// Секретный ключ Ed25519
    pub signing_key: ed25519_dalek::SigningKey,
}

static CONFIG: OnceCell<Config> = OnceCell::new();

impl Config {
    /// Инициализировать глобальную конфигурацию. Повторный вызов - ошибка
    pub fn init(encryption_key: [u8; 32], signing_key: ed25519_dalek::SigningKey) {
        CONFIG
            .set(Config {
                encryption_key,
                signing_key,
            })
            .expect("Config already initialized");
    }

    /// Получить ссылку на конфигурацию
    pub fn global() -> &'static Config {
        CONFIG.get().expect("Config not initialized")
    }
}

/// Сгенерировать набор ключей "на лету"
pub fn generate_keys() -> ( [u8; 32], ed25519_dalek::SigningKey ) {
    let mut key = [0u8; 32];
    OsRng.fill_bytes(&mut key);
    let signing_key = ed25519_dalek::SigningKey::generate(&mut OsRng);
    (key, signing_key)
}
