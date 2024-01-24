use self::surreal::DbError;

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

fn non_unique_value_on_index(error: &surrealdb::Error) -> bool {
    match error {
        surrealdb::Error::Db(surrealdb::error::Db::IndexExists {
            thing: _,
            index: _,
            value: _,
        }) => true,
        surrealdb::Error::Api(surrealdb::error::Api::Query(x)) => {
            x.contains("Database index")
                && x.contains("already contains")
                && x.contains("with record")
        }
        _ => false,
    }
}

pub trait IsNonUnique<T> {
    fn is_non_unique(&self) -> bool;
}

impl<T> IsNonUnique<T> for RepoResult<T> {
    fn is_non_unique(&self) -> bool {
        match self {
            Err(RepoError::Surreal(DbError::Database(e))) => non_unique_value_on_index(&e),
            Err(RepoError::Surreal(DbError::DatabaseCheck(es))) => {
                for e in es.values() {
                    if non_unique_value_on_index(&e) {
                        return true;
                    }
                }
                false
            }
            Ok(_) => false,
            Err(_) => false,
        }
    }
}
