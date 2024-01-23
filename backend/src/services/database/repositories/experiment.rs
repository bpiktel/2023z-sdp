use axum::{
    async_trait,
    extract::{FromRef, FromRequestParts},
    http::request::Parts,
};
use hyper::StatusCode;
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::services::database::{
    surreal::{BetterCheck, MapToNotFound, SurrealDb, WithId},
    RepoResult,
};

pub struct ExperimentRepository {
    pub surreal: SurrealDb,
}

impl ExperimentRepository {
    /// Create a new experiment and return it with an identifier
    pub async fn create(&self, experiment: Experiment) -> RepoResult<WithId<Experiment>> {
        let mut result = self
            .surreal
            .query("begin")
            .query("let $experiment = create only experiment content { name: $name }")
            .query(
                r"
                for $sample_id in $sample_ids {
                    let $sample = select value id from only sample where meta::id(id) is $sample_id;
                    relate ($experiment)->experiment_sample->($sample);
                }
                ",
            )
            .query("commit")
            .query("select *, (select value meta::id(out) from ->experiment_sample) as sample_ids from only experiment where id is $experiment.id")
            .bind(("name", &experiment.name))
            .bind(("sample_ids", &experiment.sample_ids))
            .await?
            .better_check()?;
        let experiment = result.take::<Option<WithId<Experiment>>>(2)?.found()?;
        Ok(experiment)
    }

    /// Return a specific experiment
    pub async fn info(&self, experiment_id: String) -> RepoResult<WithId<Experiment>> {
        let mut result = self
            .surreal
            .query("select *, (select value meta::id(out) from ->experiment_sample) as sample_ids from experiment where meta::id(id) is $experiment_id")
            .bind(("experiment_id", experiment_id))
            .await?;
        let experiment = result.take::<Option<WithId<Experiment>>>(0)?.found()?;
        Ok(experiment)
    }

    /// Return existing experiments
    pub async fn infos(&self) -> RepoResult<Vec<WithId<Experiment>>> {
        let mut result = self
            .surreal
            .query("select *, (select value meta::id(out) from ->experiment_sample) as sample_ids from experiment")
            .await?;
        let experiments = result.take::<Vec<WithId<Experiment>>>(0)?;
        Ok(experiments)
    }

    /// Return all results for an experiment
    pub async fn create_result(
        &self,
        experiment_id: String,
        result: ExperimentResult,
    ) -> RepoResult<WithId<ExperimentResult>> {
        let mut result = self
            .surreal
            .query("begin")
            .query("let $result = create only result content { experiment_id: $experiment_id, training: $training, user: $user }")
            .query(
                r"
                for $sample_result in $sample_results {
                    let $experiment_sample = select value id from only experiment_sample where meta::id(in) is $experiment_id and meta::id(out) is $sample_result.sample_id;
                    relate ($experiment_sample)->sample_result->($result) content { azimuth: $sample_result.azimuth , elevation: $sample_result.elevation };
                }
                ",
            )
            .query("commit")
            .query("select *, (select meta::id(in.out) as sample_id, azimuth, elevation from <-sample_result) as sample_results from only result where id is $result.id")
            .bind(("experiment_id", experiment_id))
            .bind(("training", result.training))
            .bind(("user", result.user))
            .bind(("sample_results", result.sample_results))
            .await?
            .better_check()?;
        let result = result
            .take::<Option<WithId<ExperimentResult>>>(2)?
            .found()?;
        Ok(result)
    }

    /// Return all results for an experiment
    pub async fn results(
        &self,
        experiment_id: String,
    ) -> RepoResult<Vec<WithId<ExperimentResult>>> {
        let mut result = self
            .surreal
            .query("select *, (select meta::id(in.out) as sample_id, azimuth, elevation from <-sample_result) as sample_results from result where experiment_id is $experiment_id")
            .bind(("experiment_id", experiment_id))
            .await?;
        let results = result.take::<Vec<WithId<ExperimentResult>>>(0)?;
        Ok(results)
    }

    /// Delete the entire experiment
    pub async fn delete(&self, experiment_id: String) -> RepoResult {
        self.surreal
            .query("delete from experiment where meta::id(id) is $experiment_id")
            .bind(("experiment_id", &experiment_id))
            .await?;
        Ok(())
    }
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct Experiment {
    #[validate(length(min = 1, max = 63))]
    pub name: String,
    pub sample_ids: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct ExperimentResult {
    training: bool,
    #[validate(length(min = 1, max = 63))]
    user: String,
    #[validate]
    sample_results: Vec<SampleResult>,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct SampleResult {
    pub sample_id: String,
    pub azimuth: f32,
    pub elevation: f32,
}

#[async_trait]
impl<S> FromRequestParts<S> for ExperimentRepository
where
    SurrealDb: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = StatusCode;

    async fn from_request_parts(_: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        Ok(Self {
            surreal: SurrealDb::from_ref(state),
        })
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use bytes::Bytes;
    use uuid::Uuid;

    use crate::services::database::{
        files::{FileStorage, FileStorageConfig},
        repositories::{
            experiment::{Experiment, ExperimentResult, SampleResult},
            sample::{SampleInfo, SampleRepository},
        },
        surreal::tests::surreal_in_memory,
    };

    use super::ExperimentRepository;

    async fn setup() -> (ExperimentRepository, SampleRepository) {
        let surreal = surreal_in_memory().await;
        let file_storage_config = FileStorageConfig {
            folder: PathBuf::from("./tmp/file_storage"),
        };
        let file_storage = FileStorage::setup(&file_storage_config).await.unwrap();

        (
            ExperimentRepository {
                surreal: surreal.clone(),
            },
            SampleRepository {
                surreal,
                file_storage,
            },
        )
    }

    #[tokio::test]
    async fn create() {
        let (sut, sample_repo) = setup().await;
        let info = SampleInfo {
            name: Uuid::new_v4().to_string(),
            azimuth: 10.0,
            elevation: 0.0,
        };
        let data = Bytes::from_static(&[7, 6, 5, 4, 3, 2, 1, 0]);
        let sample = sample_repo.create(info, data).await.unwrap();
        let experiment = Experiment {
            name: "exp-1".to_owned(),
            sample_ids: vec![sample.id()],
        };

        let experiment = sut.create(experiment).await.unwrap();

        assert_eq!(experiment.sample_ids.len(), 1);
    }

    #[tokio::test]
    async fn create_non_existing_audio() {
        let (sut, _) = setup().await;
        let experiment = Experiment {
            name: "exp-1".to_owned(),
            sample_ids: vec!["aaa".to_owned()],
        };

        sut.create(experiment).await.unwrap_err();
    }

    #[tokio::test]
    async fn info() {
        let (sut, sample_repo) = setup().await;
        let info = SampleInfo {
            name: Uuid::new_v4().to_string(),
            azimuth: 10.0,
            elevation: 0.0,
        };
        let data = Bytes::from_static(&[7, 6, 5, 4, 3, 2, 1, 0]);
        let sample = sample_repo.create(info, data).await.unwrap();
        let experiment = Experiment {
            name: "exp-1".to_owned(),
            sample_ids: vec![sample.id()],
        };
        let experiment = sut.create(experiment).await.unwrap();

        sut.info(experiment.id()).await.unwrap();
    }

    #[tokio::test]
    async fn infos() {
        let (sut, sample_repo) = setup().await;
        let info = SampleInfo {
            name: Uuid::new_v4().to_string(),
            azimuth: 10.0,
            elevation: 0.0,
        };
        let data = Bytes::from_static(&[7, 6, 5, 4, 3, 2, 1, 0]);
        let sample = sample_repo.create(info, data).await.unwrap();
        let experiment = Experiment {
            name: "exp-1".to_owned(),
            sample_ids: vec![sample.id()],
        };
        sut.create(experiment).await.unwrap();
        let experiment = Experiment {
            name: "exp-2".to_owned(),
            sample_ids: vec![sample.id()],
        };
        sut.create(experiment).await.unwrap();

        let result = sut.infos().await.unwrap();

        assert_eq!(result.len(), 2);
        assert_eq!(result[0].sample_ids.len(), 1);
        assert_eq!(result[1].sample_ids.len(), 1);
    }

    #[tokio::test]
    async fn delete() {
        let (sut, sample_repo) = setup().await;
        let info = SampleInfo {
            name: Uuid::new_v4().to_string(),
            azimuth: 10.0,
            elevation: 0.0,
        };
        let data = Bytes::from_static(&[7, 6, 5, 4, 3, 2, 1, 0]);
        let sample = sample_repo.create(info, data).await.unwrap();
        let experiment = Experiment {
            name: "exp-1".to_owned(),
            sample_ids: vec![sample.id()],
        };
        let experiment = sut.create(experiment).await.unwrap();

        sut.delete(experiment.id()).await.unwrap();
    }

    #[tokio::test]
    #[ignore = "only with docker db instance"]
    async fn create_result() {
        let (sut, sample_repo) = setup().await;
        let info = SampleInfo {
            name: Uuid::new_v4().to_string(),
            azimuth: 10.0,
            elevation: 0.0,
        };
        let data = Bytes::from_static(&[7, 6, 5, 4, 3, 2, 1, 0]);
        let sample = sample_repo.create(info, data).await.unwrap();
        let experiment = Experiment {
            name: "exp-1".to_owned(),
            sample_ids: vec![sample.id()],
        };
        let experiment = sut.create(experiment).await.unwrap();
        let result = ExperimentResult {
            training: false,
            user: String::default(),
            sample_results: vec![SampleResult {
                sample_id: sample.id(),
                azimuth: 17.0,
                elevation: 9.3,
            }],
        };

        let result = sut.create_result(experiment.id(), result).await.unwrap();

        assert_eq!(result.sample_results.len(), 1);
    }

    #[tokio::test]
    #[ignore = "only with docker db instance"]
    async fn results() {
        let (sut, sample_repo) = setup().await;
        let info = SampleInfo {
            name: Uuid::new_v4().to_string(),
            azimuth: 10.0,
            elevation: 0.0,
        };
        let data = Bytes::from_static(&[7, 6, 5, 4, 3, 2, 1, 0]);
        let sample = sample_repo.create(info, data).await.unwrap();
        let experiment = Experiment {
            name: "exp-1".to_owned(),
            sample_ids: vec![sample.id()],
        };
        let experiment = sut.create(experiment).await.unwrap();
        let result = ExperimentResult {
            training: false,
            user: String::default(),
            sample_results: vec![SampleResult {
                sample_id: sample.id(),
                azimuth: 17.0,
                elevation: 9.3,
            }],
        };
        sut.create_result(experiment.id(), result).await.unwrap();
        let result = ExperimentResult {
            training: false,
            user: String::default(),
            sample_results: vec![SampleResult {
                sample_id: sample.id(),
                azimuth: 10.3,
                elevation: 1.5,
            }],
        };
        sut.create_result(experiment.id(), result).await.unwrap();

        let result = sut.results(experiment.id()).await.unwrap();

        assert_eq!(result.len(), 2);
        assert_eq!(result[0].sample_results.len(), 1);
        assert_eq!(result[1].sample_results.len(), 1);
        assert_eq!(result[0].sample_results[0].sample_id, sample.id());
    }
}
