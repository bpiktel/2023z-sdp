use config::{Environment, File, FileFormat};
use serde::Deserialize;
use validator::Validate;

use super::{
    app::AppConfig, auth::AuthKeysConfig, database::surreal::DatabaseConfig,
    file_storage::FileStorageConfig, tracing::TracingConfig,
};

#[derive(Debug, Clone, Deserialize, Validate)]
pub struct AdminConfig {
    #[validate(length(min = 4))]
    pub username: String,
    #[validate(length(min = 4))]
    pub password: String,
}

#[derive(Debug, Clone, Deserialize, Validate)]
pub struct Config {
    pub app: AppConfig,
    pub tracing: TracingConfig,
    pub auth_keys: AuthKeysConfig,
    pub database: DatabaseConfig,
    pub file_storage: FileStorageConfig,
    #[validate]
    pub admin: AdminConfig,
}

pub fn setup_config() -> Config {
    let config = config::Config::builder()
        .add_source(File::with_name("config/app.json").format(FileFormat::Json))
        .add_source(Environment::default().separator("__").list_separator(","))
        .build()
        .expect("Failed to load application configuration")
        .try_deserialize::<Config>()
        .expect("Failed to deserialize application configuration");
    config.validate().unwrap();
    config
}
