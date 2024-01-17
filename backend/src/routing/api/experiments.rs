use axum::{
    extract::{FromRef, Path},
    routing::{delete, get, post},
    Json, Router,
};
use hyper::StatusCode;
use tracing::error;

use crate::services::{
    auth::{claims::Claims, AuthKeys},
    database::{
        files::FileStorage,
        repositories::experiment::{Experiment, ExperimentRepository, ExperimentResult},
        surreal::{SurrealDb, WithId},
    },
    util::ResponseType,
};

pub fn router<T>() -> Router<T>
where
    AuthKeys: FromRef<T>,
    SurrealDb: FromRef<T>,
    FileStorage: FromRef<T>,
    T: 'static + Send + Sync + Clone,
{
    Router::new()
        .route("/", post(create_experiment))
        .route("/", get(list_experiments))
        .route("/:id", get(get_experiment))
        .route("/:id", delete(delete_experiment))
        .route("/results/:id", get(get_results))
        .route("/results/:id", post(post_result))
}

/// Create experiment
///
/// Create an experiment.
async fn create_experiment(
    repo: ExperimentRepository,
    _: Claims,
    Json(experiment): Json<Experiment>,
) -> ResponseType<Json<WithId<Experiment>>> {
    let Ok(result) = repo
        .create(experiment)
        .await
        .map_err(|e| error!({error = ?e}, "Encountered an error while creating an experiment."))
    else {
        return ResponseType::Status(StatusCode::INTERNAL_SERVER_ERROR);
    };
    ResponseType::Data(Json(result))
}

/// Delete experiment
///
/// Delete the experiment and all of its results.
async fn delete_experiment(
    repo: ExperimentRepository,
    _: Claims,
    Path(id): Path<String>,
) -> ResponseType<()> {
    let Ok(_) = repo
        .delete(id)
        .await
        .map_err(|e| error!({error = ?e}, "Encountered an error while deleting an experiment."))
    else {
        return ResponseType::Status(StatusCode::INTERNAL_SERVER_ERROR);
    };
    ResponseType::Status(StatusCode::OK)
}

/// List all experiments
///
/// List all existing experiments.
async fn list_experiments(
    repo: ExperimentRepository,
) -> ResponseType<Json<Vec<WithId<Experiment>>>> {
    let Ok(result) = repo
        .infos()
        .await
        .map_err(|e| error!({error = ?e}, "Encountered an error while listing experiments."))
    else {
        return ResponseType::Status(StatusCode::INTERNAL_SERVER_ERROR);
    };
    ResponseType::Data(Json(result))
}

/// Get a specific experiments
///
/// Get a specific existing experiment.
async fn get_experiment(
    repo: ExperimentRepository,
    Path(id): Path<String>,
) -> ResponseType<Json<WithId<Experiment>>> {
    let Ok(result) = repo
        .info(id)
        .await
        .map_err(|e| error!({error = ?e}, "Encountered an error while getting an experiment."))
    else {
        return ResponseType::Status(StatusCode::INTERNAL_SERVER_ERROR);
    };
    ResponseType::Data(Json(result))
}

/// Get experiment results
///
/// Get all experiment results for the experiment.
async fn get_results(
    repo: ExperimentRepository,
    _: Claims,
    Path(id): Path<String>,
) -> ResponseType<Json<Vec<WithId<ExperimentResult>>>> {
    let Ok(result) = repo.results(id).await.map_err(
        |e| error!({error = ?e}, "Encountered an error while getting experiment results."),
    ) else {
        return ResponseType::Status(StatusCode::INTERNAL_SERVER_ERROR);
    };
    ResponseType::Data(Json(result))
}

/// Create experiment result
///
/// Create an experiment result for the experiment.
async fn post_result(
    repo: ExperimentRepository,
    Path(id): Path<String>,
    Json(expr): Json<ExperimentResult>,
) -> ResponseType<Json<WithId<ExperimentResult>>> {
    let Ok(result) = repo.create_result(id, expr).await.map_err(
        |e| error!({error = ?e}, "Encountered an error while creating experiment results."),
    ) else {
        return ResponseType::Status(StatusCode::INTERNAL_SERVER_ERROR);
    };
    ResponseType::Data(Json(result))
}
