use axum::{
    async_trait,
    extract::{FromRef, FromRequestParts},
    http::request::Parts,
};
use bytes::Bytes;
use hyper::StatusCode;
use serde::Serialize;
use utoipa::ToSchema;

use crate::services::database::{files::FileStorage, surreal::SurrealDb};

pub struct AudioRepository {
    surreal: SurrealDb,
    file_storage: FileStorage,
}

impl AudioRepository {
    /// Create sample
    pub async fn create_sample(
        &self,
        name: String,
        data: Bytes,
        azimuth: f32,
        elevation: f32,
    ) -> Result<AudioSample, ()> {
        todo!();
    }

    /// Create many samples
    pub async fn create_samples(
        &self,
        datas: Vec<(String, Bytes, f32, f32)>,
    ) -> Result<Vec<AudioSample>, ()> {
        todo!();
    }

    /// List samples
    pub async fn list_samples(&self) -> Result<Vec<AudioSample>, ()> {
        // TODO errors :)
        todo!();
    }

    /// Delete sample
    pub async fn delete_sample(&self, name: String) -> Result<AudioSample, ()> {
        todo!();
    }

    /// Delete many samples
    pub async fn delete_samples(&self, ids: Vec<String>) -> Result<(), ()> {
        todo!();
    }

    /// Get sample data
    pub async fn get_sample_data(&self, id: String) -> Result<Bytes, ()> {
        todo!();
    }
}

#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct AudioSample {
    id: String,
    name: String,
    azimuth: f32,
    elevation: f32,
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
