//! Api server

pub mod auth;

use axum::{extract::FromRef, http::StatusCode, response::IntoResponse, Router};

use crate::services::auth::AuthKeys;

use self::auth::auth_router;

pub fn api_router<T>() -> Router<T>
where
    AuthKeys: FromRef<T>,
    T: 'static + Send + Sync + Clone,
{
    Router::new()
        .nest("/auth", auth_router())
        .fallback(handler_404)
}

async fn handler_404() -> impl IntoResponse {
    StatusCode::NOT_FOUND
}
