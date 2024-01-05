pub mod files;
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
