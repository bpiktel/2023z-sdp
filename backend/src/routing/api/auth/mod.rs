use axum::{extract::FromRef, routing::post, Router};

use crate::services::{auth::AuthKeys, database::surreal::SurrealDb};

use self::login::login;

mod login;

pub fn auth_router<T>() -> Router<T>
where
    AuthKeys: FromRef<T>,
    SurrealDb: FromRef<T>,
    T: 'static + Send + Sync + Clone,
{
    Router::new().route("/login", post(login))
}
