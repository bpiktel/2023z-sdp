use std::net::SocketAddr;

use axum::extract::FromRef;
use serde::Deserialize;

use super::{
    auth::AuthKeys,
    database::{files::FileStorage, surreal::SurrealDb},
};

#[derive(FromRef, Clone)]
pub struct AppState {
    pub auth_keys: AuthKeys,
    pub surreal_db: SurrealDb,
    pub file_storage: FileStorage,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AppConfig {
    pub url: SocketAddr,
    pub permissive_cors: bool,
}
