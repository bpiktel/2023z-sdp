use axum::{
    extract::FromRef,
    routing::{get, post},
    Json, Router,
};
use axum_extra::extract::Multipart;
use hyper::StatusCode;
use serde::Deserialize;
use tracing::error;
use validator::Validate;
use utoipa::{IntoParams, ToSchema};

use crate::services::{
    auth::{
        claims::Claims,
        AuthKeys,
    },
    database::{
        files::FileStorage,
        repositories::audio::{AudioRepository, AudioSample},
        surreal::SurrealDb,
    },
    util::{ResponseType, ValidatedJson},
};

pub fn audio_router<T>() -> Router<T>
where
    AuthKeys: FromRef<T>,
    SurrealDb: FromRef<T>,
    FileStorage: FromRef<T>,
    T: 'static + Send + Sync + Clone,
{
    Router::new()
        .route("/", post(create_audio))
        .route("/delete", post(delete_audio))
        .route("/get", get(get_audio))
}

#[derive(Debug, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct CreateSamplesRequest {
    samples: Vec<CreateSampleRequest>,
}

#[derive(Debug, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct CreateSampleRequest {
    name: String,
    azimuth: f32,
    elevation: f32,
}

#[derive(Debug, Deserialize, IntoParams, ToSchema, Validate)]
#[serde(rename_all = "camelCase")]
pub struct DeleteSamplesRequest {
    ids: Vec<String>,
}

/// Create audio samples from a list
///
/// Upload audio samples to file storage and insert metadata into the database. Return sample indentifiers.
#[utoipa::path(
    post,
    path = "/audio",
    responses(
        (status = 200, description = "Upload all audio samples successfully", body = [AudioSample])
    )
)]
async fn create_audio(
    audio_repo: AudioRepository,
    _: Claims,
    mut multipart: Multipart,
) -> ResponseType<Json<Vec<AudioSample>>> {
    /// TODO multipart :)
    todo!();
    /*
    let Ok(sample) = audio_repo
        .create_samples(data)
        .await
        .map_err(|e| error!({error = ?e}, "Encountered an error while adding a sample"))
    else {
        return ResponseType::Status(StatusCode::INTERNAL_SERVER_ERROR);
    };

    ResponseType::Data(Json(sample))
    */
}

/// Delete a list of audio samples
///
/// Delete all samples with given identifiers.
#[utoipa::path(
    post,
    path = "/audio/delete",
    responses(
        (status = 200, description = "Delete listed audio samples successfully", body = DeleteSamplesRequest)
    )
)]
async fn delete_audio(
    audio_repo: AudioRepository,
    _: Claims,
    ValidatedJson(data): ValidatedJson<DeleteSamplesRequest>,
) -> ResponseType<()> {
    let Ok(_) = audio_repo
        .delete_samples(data.ids)
        .await
        .map_err(|e| error!({error = ?e}, "Encountered an error while deleting a sample"))
    else {
        return ResponseType::Status(StatusCode::INTERNAL_SERVER_ERROR);
    };

    ResponseType::Status(StatusCode::OK)
}

/// List all audio samples
///
/// list all available audio sample identifiers
#[utoipa::path(
    post,
    path = "/audio/get",
    responses(
        (status = 200, description = "List all audio samples successfully", body = [AudioSample])
    )
)]
async fn get_audio(audio_repo: AudioRepository, _: Claims) -> ResponseType<Json<Vec<AudioSample>>> {
    let Ok(samples) = audio_repo
        .list_samples()
        .await
        .map_err(|e| error!({error = ?e}, "Encountered an error while getting sample list"))
    else {
        return ResponseType::Status(StatusCode::INTERNAL_SERVER_ERROR);
    };

    ResponseType::Data(Json(samples))
}
