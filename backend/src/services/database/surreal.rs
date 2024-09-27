use serde::Deserialize;
use std::ops::Deref;
use surrealdb::{
    engine::any::{connect, Any},
    Surreal,
};

use super::{error::DbError, migrator::MigratorConfig};

#[derive(Debug, Clone, Deserialize)]
pub struct DatabaseConfig {
    pub address: String,
    pub namespace: String,
    pub database: String,
    pub migrations: MigratorConfig,
}

#[derive(Debug, Clone)]
pub struct Database {
    inner: Surreal<Any>,
}

impl Database {
    pub async fn setup(config: &DatabaseConfig) -> DbResult<Self> {
        let inner = connect(&config.address).await.unwrap();
        inner
            .use_ns(&config.namespace)
            .use_db(&config.database)
            .await?;
        Ok(Database { inner })
    }
}

impl Deref for Database {
    type Target = Surreal<Any>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
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

#[cfg(test)]
pub mod tests {
    use surrealdb::engine::any::connect;

    use crate::services::database::migrator::{Migrator, MigratorConfig};

    use super::Database;

    fn memory() -> &'static str {
        "mem://"
    }

    #[allow(dead_code)]
    fn docker() -> &'static str {
        "ws://127.0.0.1:8000"
    }

    async fn clear_db(db: &Database) {
        db.query(
            r"
            remove table experiment;
            remove table experiment_sample;
            remove table sample;
            remove table sample_result;
            remove table result;
            ",
        )
        .await
        .unwrap();
    }

    async fn migrate_db(db: &Database) {
        Migrator::new(&MigratorConfig {
            directory: "./migrations".into(),
        })
        .migrate(db)
        .await
        .unwrap();
    }

    pub async fn surreal_in_memory() -> Database {
        let addr = memory();
        let db = Database {
            inner: connect(addr).await.unwrap(),
        };
        db.use_ns("test").use_db("test").await.unwrap();
        clear_db(&db).await;
        migrate_db(&db).await;
        db
    }
}
