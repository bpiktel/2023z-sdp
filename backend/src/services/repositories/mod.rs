use super::{
    database::{error::DbError, identified::IdConversionError},
    file_storage::FsError,
};

pub mod experiment;
pub mod sample;
pub mod user;

#[derive(Debug, thiserror::Error)]
pub enum RepoError {
    #[error("File Storage: {0}")]
    FileStorage(#[from] FsError),
    #[error("Database: {0}")]
    Database(#[from] DbError),
    #[error("Id Covnersion: {0}")]
    IdConversion(#[from] IdConversionError),
}

impl From<surrealdb::Error> for RepoError {
    fn from(value: surrealdb::Error) -> Self {
        Self::Database(value.into())
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

pub trait IsViolatingUnique<T> {
    fn is_violating_unique(&self) -> bool;
}

impl<T> IsViolatingUnique<T> for RepoResult<T> {
    fn is_violating_unique(&self) -> bool {
        match self {
            Err(RepoError::Database(DbError::Query(e))) => non_unique_value_on_index(e),
            Err(_) => false,
            Ok(_) => false,
        }
    }
}
