use axum::{
    async_trait,
    extract::{FromRef, FromRequestParts},
    http::request::Parts,
};
use hyper::StatusCode;

use crate::services::database::surreal::SurrealDb;

pub struct UserRepository {
    surreal: SurrealDb,
}

impl UserRepository {
    /// Create admin account if it doesn't exist
    pub async fn try_create(&self, username: &str, password: &str) -> Result<(), ()> {
        todo!();
    }

    /// Returns whether login data identify an admin
    pub async fn is_admin(&self, username: &str, password: &str) -> Result<bool, ()> {
        todo!();
    }
}

#[async_trait]
impl<S> FromRequestParts<S> for UserRepository
where
    SurrealDb: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = StatusCode;

    async fn from_request_parts(_: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        Ok(Self {
            surreal: SurrealDb::from_ref(state),
        })
    }
}
