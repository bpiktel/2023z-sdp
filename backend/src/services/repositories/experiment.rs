use axum::{
    async_trait,
    extract::{FromRef, FromRequestParts},
    http::request::Parts,
};
use hyper::StatusCode;
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::services::database::{
    error::ValidateDbResponse,
    identified::{Identified, StringIdentified, TryIntoStringId},
    surreal::{Database, MapToNotFound},
};

use super::RepoResult;

pub struct ExperimentRepository {
    pub surreal: Database,
}

impl ExperimentRepository {
    /// Create a new experiment and return it with an identifier
    pub async fn create(&self, experiment: Experiment) -> RepoResult<StringIdentified<Experiment>> {
        let mut result = self
            .surreal
            .query("begin")
            .query("let $exp = create only experiment content { name: $experiment.name, is_public: $experiment.is_public } RETURN AFTER")
            .query(
                r"
                for $sample_id in $experiment.sample_ids {
                    let $sample = select value id from only sample where record::id(id) is $sample_id limit 1;
                    relate ($exp)->experiment_sample->($sample);
                }
                ",
            )
            .query("commit")
            .query("select *, (select value record::id(out) from ->experiment_sample) as sample_ids from only experiment where id is $exp.id limit 1")
            .bind(("experiment", experiment.clone()))
            .await?
            .validate()?;
        let experiment = result
            .take::<Option<Identified<Experiment>>>(2)?
            .found()?
            .try_into_string_id()?;
        Ok(experiment)
    }

    /// Return a specific experiment
    pub async fn info(&self, experiment_id: String) -> RepoResult<StringIdentified<Experiment>> {
        let mut result = self
            .surreal
            .query("select *, (select value record::id(out) from ->experiment_sample) as sample_ids from experiment where record::id(id) is $experiment_id")
            .bind(("experiment_id", experiment_id))
            .await?;
        let experiment = result
            .take::<Option<Identified<Experiment>>>(0)?
            .found()?
            .try_into_string_id()?;
        Ok(experiment)
    }

    /// Return existing public experiments
    pub async fn public_infos(&self) -> RepoResult<Vec<StringIdentified<Experiment>>> {
        let mut result = self
            .surreal
            .query("select *, (select value record::id(out) from ->experiment_sample) as sample_ids from experiment where is_public is true")
            .await?;
        let experiments = result
            .take::<Vec<Identified<Experiment>>>(0)?
            .try_into_string_id()?;
        Ok(experiments)
    }

    /// Return existing experiments
    pub async fn infos(&self) -> RepoResult<Vec<StringIdentified<Experiment>>> {
        let mut result = self
            .surreal
            .query("select *, (select value record::id(out) from ->experiment_sample) as sample_ids from experiment")
            .await?;
        let experiments = result
            .take::<Vec<Identified<Experiment>>>(0)?
            .try_into_string_id()?;
        Ok(experiments)
    }

    /// Return all results for an experiment
    pub async fn create_result(
        &self,
        experiment_id: String,
        result: ExperimentResult,
    ) -> RepoResult<StringIdentified<ExperimentResult>> {
        let mut result = self
            .surreal
            .query("begin")
            .query("let $result = create only result content { experiment_id: $experiment_id, training: $training, user: $user }")
            .query(
                r"
                for $sample_result in $sample_results {
                    let $experiment_sample = select value id from only experiment_sample where record::id(in) is $experiment_id and record::id(out) is $sample_result.sample_id limit 1;
                    relate ($experiment_sample)->sample_result->($result) content { azimuth: $sample_result.azimuth , elevation: $sample_result.elevation };
                }
                ",
            )
            .query("commit")
            .query("select *, (select record::id(in.out) as sample_id, azimuth, elevation from <-sample_result) as sample_results from only result where id is $result.id limit 1")
            .bind(("experiment_id", experiment_id))
            .bind(("training", result.training))
            .bind(("user", result.user))
            .bind(("sample_results", result.sample_results))
            .await?
            .validate()?;
        let result = result
            .take::<Option<Identified<ExperimentResult>>>(2)?
            .found()?
            .try_into_string_id()?;
        Ok(result)
    }

    /// Return all results for an experiment
    pub async fn results(
        &self,
        experiment_id: String,
    ) -> RepoResult<Vec<StringIdentified<ExperimentResult>>> {
        let mut result = self
            .surreal
            .query("select *, (select record::id(in.out) as sample_id, azimuth, elevation from <-sample_result) as sample_results from result where experiment_id is $experiment_id")
            .bind(("experiment_id", experiment_id))
            .await?;
        let results = result
            .take::<Vec<Identified<ExperimentResult>>>(0)?
            .try_into_string_id()?;
        Ok(results)
    }

    /// Delete the entire experiment
    pub async fn delete(&self, experiment_id: String) -> RepoResult {
        self.surreal
            .query("delete from experiment where record::id(id) is $experiment_id")
            .bind(("experiment_id", experiment_id))
            .await?;
        Ok(())
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, Validate)]
pub struct Experiment {
    #[validate(length(min = 1, max = 63))]
    pub name: String,
    pub sample_ids: Vec<String>,
    pub is_public: bool,
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
    Database: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = StatusCode;

    async fn from_request_parts(_: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        Ok(Self {
            surreal: Database::from_ref(state),
        })
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use bytes::Bytes;
    use uuid::Uuid;

    use crate::services::{
        database::surreal::tests::surreal_in_memory,
        file_storage::{FileStorage, FileStorageConfig},
        repositories::{
            experiment::{Experiment, ExperimentResult, SampleResult},
            sample::{SampleInfo, SampleRepository},
        },
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
                database: surreal,
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
            sample_ids: vec![sample.id],
            is_public: false,
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
            is_public: false,
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
            sample_ids: vec![sample.id],
            is_public: false,
        };
        let experiment = sut.create(experiment).await.unwrap();

        sut.info(experiment.id).await.unwrap();
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
            sample_ids: vec![sample.id.clone()],
            is_public: true,
        };
        sut.create(experiment).await.unwrap();
        let experiment = Experiment {
            name: "exp-2".to_owned(),
            sample_ids: vec![sample.id],
            is_public: true,
        };
        sut.create(experiment).await.unwrap();

        let result = sut.public_infos().await.unwrap();

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
            sample_ids: vec![sample.id],
            is_public: false,
        };
        let experiment = sut.create(experiment).await.unwrap();

        sut.delete(experiment.id).await.unwrap();
    }

    #[tokio::test]
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
            sample_ids: vec![sample.id.clone()],
            is_public: false,
        };
        let experiment = sut.create(experiment).await.unwrap();
        let result = ExperimentResult {
            training: false,
            user: String::default(),
            sample_results: vec![SampleResult {
                sample_id: sample.id,
                azimuth: 17.0,
                elevation: 9.3,
            }],
        };

        let result = sut.create_result(experiment.id, result).await.unwrap();

        assert_eq!(result.sample_results.len(), 1);
    }

    #[tokio::test]
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
            sample_ids: vec![sample.id.clone()],
            is_public: false,
        };
        let experiment = sut.create(experiment).await.unwrap();
        let result = ExperimentResult {
            training: false,
            user: String::default(),
            sample_results: vec![SampleResult {
                sample_id: sample.id.clone(),
                azimuth: 17.0,
                elevation: 9.3,
            }],
        };
        sut.create_result(experiment.id.clone(), result)
            .await
            .unwrap();
        let result = ExperimentResult {
            training: false,
            user: String::default(),
            sample_results: vec![SampleResult {
                sample_id: sample.id.clone(),
                azimuth: 10.3,
                elevation: 1.5,
            }],
        };
        sut.create_result(experiment.id.clone(), result)
            .await
            .unwrap();

        let result = sut.results(experiment.id).await.unwrap();

        assert_eq!(result.len(), 2);
        assert_eq!(result[0].sample_results.len(), 1);
        assert_eq!(result[1].sample_results.len(), 1);
        assert_eq!(result[0].sample_results[0].sample_id, sample.id);
    }
}
