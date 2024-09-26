use axum::{
    extract::{FromRef, Path},
    routing::{delete, get, post},
    Json, Router,
};
use hyper::StatusCode;
use tracing::error;

use crate::services::{
    auth::{
        claims::{Claims, OptClaims},
        AuthKeys,
    },
    database::{
        files::FileStorage,
        repositories::experiment::{Experiment, ExperimentRepository, ExperimentResult},
        surreal::{SurrealDb, WithId},
        IsNonUnique,
    },
    util::{ResponseType, ValidatedJson},
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
    ValidatedJson(experiment): ValidatedJson<Experiment>,
) -> ResponseType<Json<WithId<Experiment>>> {
    let result = repo.create(experiment).await.map_err(|e| {
        error!({error = ?e}, "Encountered an error while creating an experiment.");
        e
    });
    if result.is_non_unique() {
        ResponseType::Status(StatusCode::CONFLICT)
    } else if let Ok(result) = result {
        ResponseType::Data(Json(result))
    } else {
        ResponseType::Status(StatusCode::INTERNAL_SERVER_ERROR)
    }
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

/// List experiments
///
/// List existing experiments.
/// If user is logged in lists all experiments.
/// If user is not logged in, lists only public experiments.
async fn list_experiments(
    repo: ExperimentRepository,
    claims: OptClaims,
) -> ResponseType<Json<Vec<WithId<Experiment>>>> {
    let result = if claims.logged_in() {
        repo.infos().await
    } else {
        repo.public_infos().await
    }
    .map_err(|e| error!({error = ?e}, "Encountered an error while listing experiments."));

    let Ok(result) = result else {
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
    ValidatedJson(expr): ValidatedJson<ExperimentResult>,
) -> ResponseType<Json<WithId<ExperimentResult>>> {
    let Ok(result) = repo.create_result(id, expr).await.map_err(
        |e| error!({error = ?e}, "Encountered an error while creating experiment results."),
    ) else {
        return ResponseType::Status(StatusCode::INTERNAL_SERVER_ERROR);
    };
    ResponseType::Data(Json(result))
}
