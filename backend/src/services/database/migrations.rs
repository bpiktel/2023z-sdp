use std::{
    fs::{read_dir, DirEntry},
    path::PathBuf,
};

use bytes::{Bytes, BytesMut};
use chrono::{DateTime, Utc};
use serde::Deserialize;
use sha3::{Digest, Sha3_256};
use surrealdb::{
    sql::{Id, Thing},
    Connection, Surreal,
};
use tracing::{error, info};

#[derive(Debug, Clone, Deserialize)]
pub struct MigratorConfig {
    pub directory: PathBuf,
}

pub struct Migrator<'a> {
    config: &'a MigratorConfig,
}

impl<'a> Migrator<'a> {
    pub fn new(config: &'a MigratorConfig) -> Self {
        Self { config }
    }
}

#[derive(Debug, Deserialize)]
struct AppliedMigrationRaw {
    id: Thing,
    applied_at: DateTime<Utc>,
    hash: String,
}

impl From<AppliedMigrationRaw> for AppliedMigration {
    fn from(value: AppliedMigrationRaw) -> Self {
        let Id::String(id) = value.id.id else {
            panic!();
        };
        Self {
            id,
            applied_at: value.applied_at,
            hash: value.hash,
        }
    }
}

#[derive(Debug, Clone)]
struct AppliedMigration {
    id: String,
    applied_at: DateTime<Utc>,
    hash: String,
}

#[derive(Debug, Clone)]
struct Migration {
    id: String,
    query: String,
    hash: String,
}

impl<'a> Migrator<'a> {
    async fn get_applied_migrations<T: Connection>(
        &self,
        db: &Surreal<T>,
    ) -> surrealdb::Result<Vec<AppliedMigration>> {
        let migrations_raw: Vec<AppliedMigrationRaw> = db.select("migrations").await?;
        let migrations = migrations_raw
            .into_iter()
            .map(AppliedMigration::from)
            .collect();
        Ok(migrations)
    }

    fn sha256(data: &[u8]) -> Bytes {
        let mut hasher = Sha3_256::new();
        hasher.update(data);
        let result = hasher.finalize();
        BytesMut::from(result.as_slice()).freeze()
    }

    fn get_all_migrations(&self) -> MigrationResult<Vec<Migration>> {
        let dir_reader = read_dir(&self.config.directory)?;
        let migration_files: Vec<DirEntry> = dir_reader.collect::<std::io::Result<_>>()?;
        let mut migration_files: Vec<Migration> = migration_files
            .into_iter()
            .map(|entry| {
                let id = entry.file_name().to_string_lossy().into_owned();
                let query = std::fs::read_to_string(entry.path())?;
                let hash = hex::encode(Self::sha256(query.as_bytes()));
                Ok(Migration { id, query, hash })
            })
            .collect::<MigrationResult<_>>()?;
        migration_files.sort_by(|a, b| a.id.cmp(&b.id));
        Ok(migration_files)
    }

    fn migrations_to_apply<'b>(
        applied_migrations: &[AppliedMigration],
        all_migrations: &'b [Migration],
    ) -> MigrationResult<&'b [Migration]> {
        let applied_count = applied_migrations.len();
        if applied_count > all_migrations.len() {
            Err(Error::Mismatch)?
        }
        let mut applied_migrations: Vec<_> = applied_migrations.to_vec();
        applied_migrations.sort_by(|a, b| a.id.cmp(&b.id));
        for (i, applied_migration) in applied_migrations.iter().enumerate() {
            let migration_id = &all_migrations[i];
            if migration_id.id != applied_migration.id {
                Err(Error::Mismatch)?
            }
            if migration_id.hash != applied_migration.hash {
                Err(Error::Mismatch)?
            }
            info!(
                "Migration `{}` was applied at `{}`",
                &applied_migration.id, &applied_migration.applied_at
            )
        }
        let to_apply = &all_migrations[applied_count..];
        Ok(to_apply)
    }

    async fn apply_migration<T: Connection>(
        &self,
        db: &Surreal<T>,
        migration: &Migration,
    ) -> MigrationResult<()> {
        const PRE: &str = r#"BEGIN TRANSACTION;
"#;
        const POST: &str = r#"INSERT INTO migrations (id, applied_at, hash) VALUES (type::thing(migrations, $mig_id), time::now(), $mig_hash);
COMMIT TRANSACTION;
"#;

        let query = PRE.to_owned() + &migration.query + POST;
        db.query(&query)
            .bind(("mig_id", migration.id.clone()))
            .bind(("mig_hash", migration.hash.clone()))
            .await
            .map_err(|e| {
                error!(error = ?e, query = &query);
                e
            })?;
        info!("Applied new migration `{}`", &migration.id);
        Ok(())
    }

    pub async fn migrate<T: Connection>(&self, db: &Surreal<T>) -> MigrationResult<()> {
        let applied_migrations = self.get_applied_migrations(db).await?;
        let migration_files = self.get_all_migrations()?;
        let migration_files = Self::migrations_to_apply(&applied_migrations, &migration_files)?;
        for migration_file in migration_files {
            self.apply_migration(db, migration_file).await?;
        }
        Ok(())
    }
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("{0}")]
    SurrealDB(#[from] surrealdb::Error),
    #[error("{0}")]
    IO(#[from] std::io::Error),
    #[error("Applied migrations do not match provided migrations")]
    Mismatch,
}

pub type MigrationResult<T> = Result<T, Error>;
