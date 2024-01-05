use axum::{
    async_trait,
    extract::{FromRef, FromRequestParts},
    http::request::Parts,
};
use bytes::Bytes;
use hyper::StatusCode;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::services::database::{
    files::FileStorage,
    surreal::{MapToNotFound, SurrealDb, WithId},
    RepoResult,
};

pub struct AudioRepository {
    surreal: SurrealDb,
    file_storage: FileStorage,
}

impl AudioRepository {
    /// Create sample
    pub async fn create(&self, info: SampleInfo, data: Bytes) -> RepoResult<WithId<SampleInfo>> {
        let mut result = self
            .surreal
            .query("create only sample content $info")
            .bind(("info", &info))
            .await?;
        let sample = result.take::<Option<WithId<SampleInfo>>>(0)?.found()?;
        self.file_storage.create(sample.id(), data).await?;
        Ok(sample)
    }

    /// List sample infos
    pub async fn infos(&self) -> RepoResult<Vec<WithId<SampleInfo>>> {
        let mut result = self.surreal.query("select * from sample").await?;
        let samples = result.take::<Vec<WithId<SampleInfo>>>(0)?;
        Ok(samples)
    }

    /// Delete sample
    pub async fn delete(&self, id: String) -> RepoResult {
        self.file_storage.delete(&id).await?;
        self.surreal
            .query("delete only sample where meta::id(id) is $id")
            .bind(("id", &id))
            .await?;
        Ok(())
    }

    /// Get sample info
    pub async fn info(&self, id: String) -> RepoResult<WithId<SampleInfo>> {
        let mut result = self
            .surreal
            .query("select * from only sample where meta::id(id) is $id")
            .bind(("id", &id))
            .await?;
        let sample = result.take::<Option<WithId<SampleInfo>>>(0)?.found()?;
        Ok(sample)
    }

    /// Get sample data
    pub async fn data(&self, id: String) -> RepoResult<Bytes> {
        let data = self.file_storage.get(id).await?;
        Ok(data)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SampleInfo {
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

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use bytes::Bytes;

    use crate::services::database::{
        files::{FileStorage, FileStorageConfig},
        repositories::audio::SampleInfo,
        surreal::tests::surreal_in_memory,
    };

    use super::AudioRepository;

    async fn setup() -> AudioRepository {
        let surreal = surreal_in_memory().await;
        tokio::fs::remove_dir_all("./tmp/file_storage").await.ok();
        let file_storage_config = FileStorageConfig {
            folder: PathBuf::from("./tmp/file_storage"),
        };
        let file_storage = FileStorage::setup(&file_storage_config).await.unwrap();

        AudioRepository {
            surreal,
            file_storage,
        }
    }

    #[tokio::test]
    async fn create() {
        let sut = setup().await;
        let info = SampleInfo {
            name: "create.mp4".to_owned(),
            azimuth: 10.0,
            elevation: 0.0,
        };
        let data = Bytes::from_static(&[7, 6, 5, 4, 3, 2, 1, 0]);

        sut.create(info, data).await.unwrap();
    }

    #[tokio::test]
    async fn create_duplicate() {
        let sut = setup().await;
        let info = SampleInfo {
            name: "create.mp4".to_owned(),
            azimuth: 10.0,
            elevation: 0.0,
        };
        let data = Bytes::from_static(&[7, 6, 5, 4, 3, 2, 1, 0]);
        sut.create(info.clone(), data.clone()).await.unwrap();

        sut.create(info, data).await.unwrap_err();
    }

    #[tokio::test]
    async fn all_infos() {
        let sut = setup().await;
        let info = SampleInfo {
            name: "all_infos1.mp4".to_owned(),
            azimuth: 10.0,
            elevation: 0.0,
        };
        let data = Bytes::from_static(&[7, 6, 5, 4, 3, 2, 1, 0]);
        sut.create(info, data).await.unwrap();
        let info = SampleInfo {
            name: "all_infos2.mp4".to_owned(),
            azimuth: 0.0,
            elevation: 10.0,
        };
        let data = Bytes::from_static(&[15, 14, 13, 12, 11, 10, 9, 8]);
        sut.create(info, data).await.unwrap();

        let samples = sut.infos().await.unwrap();

        assert_eq!(samples.len(), 2);
    }

    #[tokio::test]
    async fn info() {
        let sut = setup().await;
        let info = SampleInfo {
            name: "info.mp4".to_owned(),
            azimuth: 10.0,
            elevation: 0.0,
        };
        let data = Bytes::from_static(&[7, 6, 5, 4, 3, 2, 1, 0]);
        let sample = sut.create(info, data).await.unwrap();

        let result = sut.info(sample.id()).await.unwrap();

        assert_eq!(result, sample);
    }

    #[tokio::test]
    async fn data() {
        let sut = setup().await;
        let info = SampleInfo {
            name: "data.mp4".to_owned(),
            azimuth: 10.0,
            elevation: 0.0,
        };
        let data = Bytes::from_static(&[7, 6, 5, 4, 3, 2, 1, 0]);
        let sample = sut.create(info, data.clone()).await.unwrap();

        let result_data = sut.data(sample.id()).await.unwrap();

        assert_eq!(result_data, data);
    }

    #[tokio::test]
    async fn delete() {
        let sut = setup().await;
        let info = SampleInfo {
            name: "delete.mp4".to_owned(),
            azimuth: 10.0,
            elevation: 0.0,
        };
        let data = Bytes::from_static(&[7, 6, 5, 4, 3, 2, 1, 0]);
        let sample = sut.create(info, data).await.unwrap();

        sut.delete(sample.id()).await.unwrap();
    }
}
