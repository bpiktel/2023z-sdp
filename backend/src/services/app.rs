use std::net::SocketAddr;

use axum::extract::FromRef;
use serde::Deserialize;

use super::{auth::AuthKeys, database::surreal::Database, file_storage::FileStorage};

#[derive(FromRef, Clone)]
pub struct AppState {
    pub auth_keys: AuthKeys,
    pub database: Database,
    pub file_storage: FileStorage,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AppConfig {
    pub endpoint: SocketAddr,
    pub permissive_cors: bool,
}
