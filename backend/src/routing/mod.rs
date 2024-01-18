pub mod api;
mod healthcheck;
mod static_files;

use axum::{extract::FromRef, Router};
use tower_http::{cors::CorsLayer, trace::TraceLayer};

use crate::services::{
    auth::AuthKeys,
    config::Config,
    database::{files::FileStorage, surreal::SurrealDb},
};
use tracing::info;

use self::{api::api_router, healthcheck::healthcheck_router, static_files::static_files_service};

pub const MAIN_ROUTE_PATH: &'static str = "/api";

pub fn main_route<T>(config: &Config) -> Router<T>
where
    AuthKeys: FromRef<T>,
    SurrealDb: FromRef<T>,
    FileStorage: FromRef<T>,
    T: 'static + Send + Sync + Clone,
{
    let mut router = Router::new()
        .merge(healthcheck_router())
        .nest(MAIN_ROUTE_PATH, api_router())
        .fallback_service(static_files_service())
        .layer(TraceLayer::new_for_http());

    if config.app.cors {
        router = router.layer(CorsLayer::very_permissive());
        info!("Applying very permissive CORS");
    } else {
        info!("Applying restrictive CORS");
    }

    router
}
