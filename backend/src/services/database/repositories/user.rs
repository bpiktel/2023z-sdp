use axum::{
    async_trait,
    extract::{FromRef, FromRequestParts},
    http::request::Parts,
};
use hyper::StatusCode;
use serde::{Deserialize, Serialize};
use sha3::{Digest, Keccak256};

use crate::services::database::{
    surreal::{MapToNotFound, SurrealDb, WithId},
    RepoResult,
};

pub struct UserRepository {
    surreal: SurrealDb,
}

#[derive(Debug, Serialize, Deserialize)]
struct CreateUser {
    id: String,
    username: String,
    password_hash: Vec<u8>,
}

const ADMIN_ID: &str = "admin";

impl UserRepository {
    pub fn new(surreal: SurrealDb) -> Self {
        Self { surreal }
    }

    fn hash_password(password: &str) -> Vec<u8> {
        let mut hasher = Keccak256::default();
        hasher.update(password);
        let password_hash = hasher.finalize();
        password_hash.to_vec()
    }

    /// Create admin account if it doesn't exist
    pub async fn try_create(&self, username: &str, password: &str) -> RepoResult {
        let mut result = self
            .surreal
            .query("create only user content $user")
            .bind((
                "user",
                CreateUser {
                    id: ADMIN_ID.to_owned(),
                    username: username.to_owned(),
                    password_hash: Self::hash_password(password),
                },
            ))
            .await?;
        result.take::<Option<WithId>>(0)?.found()?;
        Ok(())
    }

    /// Returns whether login data identify an admin
    pub async fn is_admin(&self, username: &str, password: &str) -> RepoResult<bool> {
        let mut result = self
            .surreal
            .query("select * from user where meta::id(id) is $id and username is $username and password_hash is $password_hash")
            .bind(("id", ADMIN_ID))
            .bind(("username", username.to_owned()))
            .bind(("password_hash", Self::hash_password(password)))
            .await?;
        let mby_user = result.take::<Option<WithId>>(0)?;
        Ok(mby_user.is_some())
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

#[cfg(test)]
mod tests {
    use crate::services::database::surreal::tests::surreal_in_memory;

    use super::UserRepository;

    async fn setup() -> UserRepository {
        let surreal = surreal_in_memory().await;

        UserRepository { surreal }
    }

    #[tokio::test]
    async fn try_create() {
        let sut = setup().await;

        sut.try_create("admin", "admin").await.unwrap();
    }

    #[tokio::test]
    async fn try_create_twice() {
        let sut = setup().await;

        sut.try_create("admin", "admin").await.unwrap();
        sut.try_create("admin2", "admin2").await.unwrap_err();
    }

    #[tokio::test]
    async fn is_admin() {
        let sut = setup().await;
        sut.try_create("admin", "admin").await.unwrap();

        let result = sut.is_admin("admin", "admin").await.unwrap();

        assert!(result);
    }

    #[tokio::test]
    async fn is_not_admin() {
        let sut = setup().await;
        sut.try_create("admin", "admin").await.unwrap();

        let result = sut.is_admin("admin2", "admin2").await.unwrap();

        assert!(!result);
    }
}
