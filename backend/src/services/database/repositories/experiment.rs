use axum::{
    async_trait,
    extract::{FromRef, FromRequestParts},
    http::request::Parts,
};
use hyper::StatusCode;
use surrealdb::{engine::any::Any, Surreal};

use crate::services::database::surreal::SurrealDb;

pub struct ExperimentRepository {
    surreal: SurrealDb,
}

impl ExperimentRepository {
    /// Create a new experiment and return it with an identifier
    pub async fn create_experiment(&self, experiment: Experiment) -> Experiment {
        todo!();
    }

    /// Return an existing experiment
    pub async fn get_experiment(&self, experiment_id: String) -> Experiment {
        todo!();
    }

    /// Return all results for an experiment
    pub async fn create_experiment_result(&self, result: ExperimentResult) -> ExperimentResult {
        todo!();
    }

    /// Return all results for an experiment
    pub async fn get_experiment_results(&self, experiment_id: String) -> Vec<ExperimentResult> {
        todo!();
    }

    /// Delete the entire experiment
    pub async fn delete_experiment(&self, experiment_id: String) {
        todo!();
    }
}

pub struct Experiment {
    pub name: String,
    pub samples: Vec<Sample>,
}

pub struct Sample {
    pub azimunth: f32,
    pub elevation: f32,
    pub sample_id: String,
}

pub struct ExperimentResult {
    pub name: String,
    pub samples: Vec<Sample>,
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
