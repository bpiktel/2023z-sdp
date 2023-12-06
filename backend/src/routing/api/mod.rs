//! Api server

pub mod audio;
pub mod auth;

use axum::{extract::FromRef, http::StatusCode, response::IntoResponse, Router};

use crate::services::{
    auth::AuthKeys,
    database::{files::FileStorage, surreal::SurrealDb},
};

use self::audio::audio_router;
use self::auth::auth_router;

pub fn api_router<T>() -> Router<T>
where
    AuthKeys: FromRef<T>,
    SurrealDb: FromRef<T>,
    FileStorage: FromRef<T>,
    T: 'static + Send + Sync + Clone,
{
    Router::new()
        .nest("/auth", auth_router())
        .nest("/audio", audio_router())
        .fallback(handler_404)
}

async fn handler_404() -> impl IntoResponse {
    StatusCode::NOT_FOUND
}
