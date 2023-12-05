use axum::{
    async_trait,
    extract::{FromRef, FromRequestParts},
    http::request::Parts,
};
use hyper::StatusCode;
use surrealdb::{engine::any::Any, Surreal};

use crate::services::database::{files::FileStorage, surreal::SurrealDb};

pub struct AudioRepository {
    surreal: SurrealDb,
    file_storage: FileStorage,
}

impl AudioRepository {
    /// Create sample and return ID
    pub async fn create_sample(&self, name: String, data: &[u8]) -> AudioSample {
        todo!();
    }

    /// Get samples
    pub async fn list_samples(&self) -> Vec<AudioSample> {
        todo!();
    }

    /// Get sample data
    pub async fn get_sample_data(&self, id: String) -> &[u8] {
        todo!();
    }
}

pub struct AudioSample {
    id: String,
    name: String,
}

#[async_trait]
impl<S> FromRequestParts<S> for AudioRepository
where
    SurrealDb: FromRef<S>,
    FileStorage: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = StatusCode;

    async fn from_request_parts(_: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        Ok(Self {
            surreal: SurrealDb::from_ref(state),
            file_storage: FileStorage::from_ref(state),
        })
    }
}
