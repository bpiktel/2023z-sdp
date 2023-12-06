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
use uuid::Uuid;

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
    pub fn id(&self) -> Uuid {
        self.id.to_uuid()
    }
}

pub trait ThingToUuid {
    fn to_uuid(&self) -> Uuid;
}

impl ThingToUuid for Thing {
    fn to_uuid(&self) -> Uuid {
        let Id::String(ref uuid) = self.id else {
            panic!("Expected string ID")
        };
        uuid.parse().expect("Failed to parse ID as UUID")
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
