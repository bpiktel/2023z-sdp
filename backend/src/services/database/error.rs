use super::migrator::MigrationError;

#[derive(Debug, thiserror::Error)]
pub enum DbError {
    #[error("Migration error: {0}")]
    Migration(#[from] MigrationError),
    #[error("Query error: {0}")]
    Query(#[from] surrealdb::Error),
    #[error("Not found")]
    NotFound,
}

pub type DbResult<T = ()> = Result<T, DbError>;

pub trait ValidateDbResponse
where
    Self: Sized,
{
    fn validate(self) -> DbResult<Self>;
}

impl ValidateDbResponse for surrealdb::Response {
    fn validate(mut self) -> DbResult<Self> {
        let mut errors = self.take_errors().into_iter().collect::<Vec<_>>();
        errors.sort_by_key(|(k, _)| *k);
        if let Some((_, error)) = errors.into_iter().next() {
            Err(DbError::Query(error))
        } else {
            Ok(self)
        }
    }
}
