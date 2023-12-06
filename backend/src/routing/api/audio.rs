use axum::{
    extract::FromRef,
    extract::State,
    routing::{get, post},
    Json, Router, debug_handler,
};
use axum_extra::extract::{cookie::Cookie, multipart, CookieJar, Multipart};
use bytes::Bytes;
use chrono::{Duration, Utc};
use hyper::StatusCode;
use serde::Deserialize;
use tracing::error;
use uuid::Uuid;
use validator::Validate;

use crate::services::{
    auth::{
        claims::{create_token, Claims, JWT_TOKEN_COOKIE},
        AuthKeys,
    },
    database::{
        files::FileStorage,
        repositories::audio::{AudioRepository, AudioSample},
        surreal::SurrealDb,
    },
    util::{ResponseType, ValidatedJson}, app::AppState,
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

#[derive(Debug, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct DeleteSamplesRequest {
    ids: Vec<String>,
}

#[debug_handler(state = AppState)]
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
