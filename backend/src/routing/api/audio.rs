use axum::{
    extract::{FromRef, Path},
    routing::{delete, get, post},
    Json, Router,
};
use axum_extra::extract::Multipart;
use hyper::StatusCode;
use serde::Deserialize;
use tracing::error;
use validator::Validate;

use crate::services::{
    auth::{claims::Claims, AuthKeys},
    database::{
        files::FileStorage,
        repositories::sample::{SampleInfo, SampleRepository},
        surreal::{SurrealDb, WithId},
    },
    util::ResponseType,
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
        .route("/delete/:id", delete(delete_audio))
        .route("/get", get(get_audio))
    // List?
    // Get data?
}

#[derive(Debug, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct CreateSampleRequest {
    name: String,
    azimuth: f32,
    elevation: f32,
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
    audio_repo: SampleRepository,
    _: Claims,
    mut multipart: Multipart,
) -> ResponseType<Json<Vec<SampleInfo>>> {
    /// TODO multipart :)
    todo!();
    /*
    let Ok(sample) = audio_repo
        .create_sample(data)
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
    delete,
    path = "/audio/delete/{id}",
    responses(
        (status = 200, description = "Delete listed audio samples successfully")
    )
)]
async fn delete_audio(
    audio_repo: SampleRepository,
    _: Claims,
    Path(id): Path<String>,
) -> ResponseType<()> {
    let Ok(_) = audio_repo
        .delete(id)
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
async fn get_audio(
    audio_repo: SampleRepository,
    _: Claims,
) -> ResponseType<Json<Vec<WithId<SampleInfo>>>> {
    let Ok(samples) = audio_repo
        .infos()
        .await
        .map_err(|e| error!({error = ?e}, "Encountered an error while getting sample list"))
    else {
        return ResponseType::Status(StatusCode::INTERNAL_SERVER_ERROR);
    };

    ResponseType::Data(Json(samples))

    // TODO: Use `mime_guess::from_path` to get MIME type from sample name
}
