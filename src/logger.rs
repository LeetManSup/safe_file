use tracing_subscriber::{fmt, EnvFilter};

/// Запустить логирование; вызывается единожды в приложении
pub fn init() {
    let filter_layer = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("info"));
    fmt().with_env_filter(filter_layer).init();
}
