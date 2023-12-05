use std::net::SocketAddr;

use axum::extract::FromRef;
use serde::Deserialize;

use super::auth::AuthKeys;

#[derive(FromRef, Clone)]
pub struct AppState {
    pub auth_keys: AuthKeys,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AppConfig {
    pub url: SocketAddr,
    pub cors: bool,
}
