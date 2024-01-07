use axum::{routing::get, Router};

pub fn healthcheck_router<T>() -> Router<T>
where
    T: 'static + Send + Sync + Clone,
{
    Router::new().route("/health", get(health))
}

async fn health() -> &'static str {
    "healthy"
}
