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

#[derive(Debug, Serialize, Deserialize)]
pub struct Record<T = ()> {
    pub id: Thing,
    #[serde(flatten)]
    pub entry: T,
}

impl<T> Record<T> {
    pub fn id(&self) -> u64 {
        self.id.to_u64()
    }
}

pub trait ThingToU64 {
    fn to_u64(&self) -> u64;
}

impl ThingToU64 for Thing {
    fn to_u64(&self) -> u64 {
        let Id::Number(id) = self.id else {
            panic!("Expected numeric id")
        };
        id.try_into().expect("Failed cast")
    }
}

impl<T> Deref for Record<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.entry
    }
}

impl<T> DerefMut for Record<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.entry
    }
}
