use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    ops::{Deref, DerefMut},
};
use surrealdb::{
    engine::any::{connect, Any},
    sql::{Id, Thing},
    Surreal,
};

#[derive(Debug, Clone, Deserialize)]
pub struct SurrealDbConfig {
    pub address: String,
    pub namespace: String,
    pub database: String,
}

#[derive(Debug, Clone)]
pub struct SurrealDb {
    inner: Surreal<Any>,
}

impl SurrealDb {
    pub async fn setup(config: &SurrealDbConfig) -> DbResult<Self> {
        let inner = connect(&config.address).await.unwrap();
        inner
            .use_ns(&config.namespace)
            .use_db(&config.database)
            .await?;
        Ok(SurrealDb { inner })
    }
}

impl Deref for SurrealDb {
    type Target = Surreal<Any>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

#[derive(Debug, thiserror::Error)]
pub enum DbError {
    #[error("{0}")]
    Database(#[from] surrealdb::Error),
    #[error("{0:?}")]
    DatabaseCheck(HashMap<usize, surrealdb::Error>),
    #[error("Not found")]
    NotFound,
}

pub type DbResult<T> = Result<T, DbError>;

pub trait MapToNotFound<T> {
    fn found(self) -> DbResult<T>;
}

impl<T> MapToNotFound<T> for Option<T> {
    fn found(self) -> DbResult<T> {
        self.ok_or(DbError::NotFound)
    }
}

pub trait BetterCheck
where
    Self: Sized,
{
    fn better_check(self) -> DbResult<Self>;
}

impl BetterCheck for surrealdb::Response {
    fn better_check(mut self) -> DbResult<Self> {
        let errors = self.take_errors();
        if errors.is_empty() {
            Ok(self)
        } else {
            Err(DbError::DatabaseCheck(errors))
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct WithId<T = ()> {
    pub id: Thing,
    #[serde(flatten)]
    pub entry: T,
}

impl<T> WithId<T> {
    pub fn id(&self) -> String {
        self.id.unwrap_string()
    }
}

pub trait ThingUnwrap {
    fn unwrap_string(&self) -> String;
}

impl ThingUnwrap for Thing {
    fn unwrap_string(&self) -> String {
        let Id::String(string) = &self.id else {
            panic!("Expected string ID")
        };
        string.to_owned()
    }
}

impl<T> Deref for WithId<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.entry
    }
}

impl<T> DerefMut for WithId<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.entry
    }
}

#[cfg(test)]
pub mod tests {
    use surrealdb::engine::any::connect;

    use super::SurrealDb;

    fn memory() -> &'static str {
        "mem://"
    }

    pub async fn surreal_in_memory() -> SurrealDb {
        let addr = memory();
        let db = SurrealDb {
            inner: connect(addr).await.unwrap(),
        };
        db.use_ns("test").use_db("test").await.unwrap();
        db
    }
}
