use std::io::ErrorKind;

use axum::{
    async_trait,
    extract::{FromRef, FromRequestParts},
    http::request::Parts,
};
use bytes::Bytes;
use hyper::StatusCode;
use serde::{Deserialize, Serialize};
use tracing::warn;
use validator::Validate;

use crate::services::database::{
    files::FileStorage,
    surreal::{MapToNotFound, SurrealDb, WithId},
    RepoResult,
};

pub struct SampleRepository {
    pub surreal: SurrealDb,
    pub file_storage: FileStorage,
}

impl SampleRepository {
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
    pub async fn delete(&self, id: String) -> RepoResult<bool> {
        let mut result = self
            .surreal
            .query("select count() from experiment_sample where meta::id(out) is $id group all")
            .bind(("id", &id))
            .await?;
        let relations_count: Option<usize> = result.take("count")?;
        if relations_count.unwrap_or(0) > 0 {
            return Ok(false);
        }
        // NOTE: Race condition
        // Someone can add this sample to an experiment between the check and deletion.
        self.file_storage.delete(&id).await.or_else(|err| {
            if err.kind() == ErrorKind::NotFound {
                warn!("{}", err);
                Ok(())
            } else {
                Err(err)
            }
        })?;
        self.surreal
            .query("delete sample where meta::id(id) is $id")
            .bind(("id", &id))
            .await?;
        Ok(true)
    }

    /// Get sample data
    pub async fn data(&self, id: String) -> RepoResult<Bytes> {
        let data = self.file_storage.get(id).await?;
        Ok(data)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct SampleInfo {
    #[validate(length(min = 1, max = 63))]
    pub name: String,
    pub azimuth: f32,
    pub elevation: f32,
}

#[async_trait]
impl<S> FromRequestParts<S> for SampleRepository
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
        repositories::{
            experiment::{Experiment, ExperimentRepository},
            sample::SampleInfo,
        },
        surreal::tests::surreal_in_memory,
    };

    use super::SampleRepository;

    async fn setup() -> (SampleRepository, ExperimentRepository) {
        let surreal = surreal_in_memory().await;
        let file_storage_config = FileStorageConfig {
            folder: PathBuf::from("./tmp/file_storage"),
        };
        let file_storage = FileStorage::setup(&file_storage_config).await.unwrap();

        (
            SampleRepository {
                surreal: surreal.clone(),
                file_storage,
            },
            ExperimentRepository { surreal },
        )
    }

    #[tokio::test]
    async fn create() {
        let (sut, _) = setup().await;
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
        let (sut, _) = setup().await;
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
        let (sut, _) = setup().await;
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
    async fn data() {
        let (sut, _) = setup().await;
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
        let (sut, _) = setup().await;
        let info = SampleInfo {
            name: "delete.mp4".to_owned(),
            azimuth: 10.0,
            elevation: 0.0,
        };
        let data = Bytes::from_static(&[7, 6, 5, 4, 3, 2, 1, 0]);
        let sample = sut.create(info, data).await.unwrap();

        sut.delete(sample.id()).await.unwrap();
    }

    #[tokio::test]
    async fn delete_fail_attachedk_to_experiment() {
        let (sut, experiment_repo) = setup().await;
        let info = SampleInfo {
            name: "delete.mp4".to_owned(),
            azimuth: 10.0,
            elevation: 0.0,
        };
        let data = Bytes::from_static(&[7, 6, 5, 4, 3, 2, 1, 0]);
        let sample = sut.create(info, data).await.unwrap();
        let experiment = Experiment {
            name: "exp-1".to_owned(),
            sample_ids: vec![sample.id()],
        };
        experiment_repo.create(experiment).await.unwrap();

        sut.delete(sample.id()).await.unwrap_err();
    }
}
