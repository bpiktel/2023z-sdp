use axum::{
    extract::{FromRef, Path},
    routing::{delete, get, post},
    Json, Router,
};
use axum_extra::extract::Multipart;
use hyper::StatusCode;
use tracing::error;
use validator::Validate;

use crate::services::{
    auth::{claims::Claims, AuthKeys},
    database::{identified::StringIdentified, surreal::Database},
    file_storage::FileStorage,
    repositories::{
        sample::{SampleInfo, SampleRepository},
        IsViolatingUnique,
    },
    util::{ResponseType, ValidatedJsonRejection},
};

pub fn audio_router<T>() -> Router<T>
where
    AuthKeys: FromRef<T>,
    Database: FromRef<T>,
    FileStorage: FromRef<T>,
    T: 'static + Send + Sync + Clone,
{
    Router::new()
        .route("/", post(create_audio))
        .route("/:id", delete(delete_audio))
        .route("/all", get(get_all))
        .route("/:id", get(get_audio))
}

/// Create audio sample
///
/// Upload an audio sample to file storage and insert metadata into the database. Return sample indentifier.
async fn create_audio(
    audio_repo: SampleRepository,
    _: Claims,
    mut multipart: Multipart,
) -> ResponseType<Json<StringIdentified<SampleInfo>>> {
    let Ok(Some(info)) = multipart
        .next_field()
        .await
        .map_err(|e| error!({error = ?e}, "Encountered an error while reading a sample"))
    else {
        return ResponseType::Status(StatusCode::INTERNAL_SERVER_ERROR);
    };
    let Ok(info_bytes) = info
        .bytes()
        .await
        .map_err(|e| error!({error = ?e}, "Encountered an error while reading a sample"))
    else {
        return ResponseType::Status(StatusCode::INTERNAL_SERVER_ERROR);
    };
    let Json(info): Json<SampleInfo> = match Json::from_bytes(&info_bytes) {
        Ok(info) => info,
        Err(err) => return ResponseType::JsonErr(ValidatedJsonRejection::Json(err)),
    };
    let Ok(Some(data)) = multipart
        .next_field()
        .await
        .map_err(|e| error!({error = ?e}, "Encountered an error while reading a sample"))
    else {
        return ResponseType::Status(StatusCode::INTERNAL_SERVER_ERROR);
    };
    let Ok(data) = data
        .bytes()
        .await
        .map_err(|e| error!({error = ?e}, "Encountered an error while reading a sample"))
    else {
        return ResponseType::Status(StatusCode::INTERNAL_SERVER_ERROR);
    };
    if let Err(err) = info.validate().map_err(|e| {
        error!({error = ?e}, "Invalid sample info");
        e
    }) {
        return ResponseType::JsonErr(ValidatedJsonRejection::Validation(err));
    }
    let result = audio_repo.create(info, data).await.map_err(|e| {
        error!({error = ?e}, "Encountered an error while adding a sample");
        e
    });
    if result.is_violating_unique() {
        ResponseType::Status(StatusCode::CONFLICT)
    } else if let Ok(result) = result {
        ResponseType::Data(Json(result))
    } else {
        ResponseType::Status(StatusCode::INTERNAL_SERVER_ERROR)
    }
}

/// Delete an audio sample
///
/// Delete an audio sample with given identifier.
async fn delete_audio(
    audio_repo: SampleRepository,
    _: Claims,
    Path(id): Path<String>,
) -> ResponseType<()> {
    let Ok(deleted) = audio_repo
        .delete(id)
        .await
        .map_err(|e| error!({error = ?e}, "Encountered an error while deleting a sample"))
    else {
        return ResponseType::Status(StatusCode::INTERNAL_SERVER_ERROR);
    };

    if deleted {
        ResponseType::Status(StatusCode::OK)
    } else {
        ResponseType::Status(StatusCode::CONFLICT)
    }
}

/// List all audio samples
///
/// List all available audio sample identifiers
async fn get_all(
    audio_repo: SampleRepository,
) -> ResponseType<Json<Vec<StringIdentified<SampleInfo>>>> {
    let Ok(samples) = audio_repo
        .infos()
        .await
        .map_err(|e| error!({error = ?e}, "Encountered an error while getting sample list"))
    else {
        return ResponseType::Status(StatusCode::INTERNAL_SERVER_ERROR);
    };

    ResponseType::Data(Json(samples))
}

/// Get audio sample data
///
/// Get raw data of an audio sample with given identifier.
async fn get_audio(
    audio_repo: SampleRepository,
    Path(id): Path<String>,
) -> ResponseType<bytes::Bytes> {
    let Ok(sample) = audio_repo
        .data(id)
        .await
        .map_err(|e| error!({error = ?e}, "Encountered an error while deleting a sample"))
    else {
        return ResponseType::Status(StatusCode::INTERNAL_SERVER_ERROR);
    };

    ResponseType::Data(sample)
}
