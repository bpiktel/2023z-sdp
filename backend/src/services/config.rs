use config::{Environment, File, FileFormat};
use serde::Deserialize;

use super::{
    app::AppConfig,
    auth::AuthKeysConfig,
    database::{files::FileStorageConfig, surreal::SurrealDbConfig},
    tracing::TracingConfig,
};

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub app: AppConfig,
    pub tracing: TracingConfig,
    pub auth_keys: AuthKeysConfig,
    pub surreal_db: SurrealDbConfig,
    pub file_storage: FileStorageConfig,
}

pub fn setup_config() -> Config {
    config::Config::builder()
        .add_source(File::with_name("config/app.json").format(FileFormat::Json))
        .add_source(Environment::default().separator("__").list_separator(","))
        .build()
        .expect("Failed to load application configuration")
        .try_deserialize::<Config>()
        .expect("Failed to deserialize application configuration")
}
