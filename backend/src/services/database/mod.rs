use self::surreal::{non_unique_value_on_index, DbError, DbResult};

pub mod files;
pub mod migrations;
pub mod repositories;
pub mod surreal;

#[derive(Debug, thiserror::Error)]
pub enum RepoError {
    #[error("Io: {0}")]
    Io(#[from] std::io::Error),
    #[error("Surreal: {0}")]
    Surreal(#[from] surreal::DbError),
}

impl From<surrealdb::Error> for RepoError {
    fn from(value: surrealdb::Error) -> Self {
        Self::Surreal(value.into())
    }
}

pub type RepoResult<T = ()> = std::result::Result<T, RepoError>;

pub trait ExtractNonUniqueIndex<T> {
    fn extract_non_unique_on_index(self, index: &str) -> RepoResult<T>;
}

impl<T> ExtractNonUniqueIndex<T> for DbResult<T> {
    fn extract_non_unique_on_index(self, index: &str) -> RepoResult<T> {
        match self {
            Err(DbError::Database(e)) => match non_unique_value_on_index(&e, index) {
                Some(v) => Err(RepoError::Surreal(DbError::NonUnique(v.to_string()))),
                None => Err(RepoError::Surreal(DbError::Database(e))),
            },
            Err(DbError::DatabaseCheck(es)) => {
                for e in es.values() {
                    if let Some(v) = non_unique_value_on_index(e, index) {
                        return Err(RepoError::Surreal(DbError::NonUnique(v.to_string())));
                    }
                }
                Err(DbError::DatabaseCheck(es).into())
            }
            Ok(v) => Ok(v),
            Err(e) => Err(e.into()),
        }
    }
}
