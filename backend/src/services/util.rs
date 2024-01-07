use axum::{
    async_trait,
    extract::{rejection::JsonRejection, FromRequest, Request},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::de::DeserializeOwned;
use validator::{Validate, ValidationErrors};

pub enum ResponseType<T = ()> {
    Data(T),
    Status(StatusCode),
    DataStatus((StatusCode, T)),
}

impl<T: IntoResponse> IntoResponse for ResponseType<T> {
    fn into_response(self) -> axum::response::Response {
        match self {
            ResponseType::Status(r) => r.into_response(),
            ResponseType::Data(r) => r.into_response(),
            ResponseType::DataStatus(r) => r.into_response(),
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ValidatedJsonRejection {
    #[error("{0}")]
    Json(#[from] JsonRejection),
    #[error("{0}")]
    Validation(#[from] ValidationErrors),
}

impl IntoResponse for ValidatedJsonRejection {
    fn into_response(self) -> axum::response::Response {
        match self {
            ValidatedJsonRejection::Json(rejection) => rejection.into_response(),
            ValidatedJsonRejection::Validation(errors) => {
                let errors = serde_json::ser::to_string(&errors).unwrap();
                (StatusCode::BAD_REQUEST, errors).into_response()
            }
        }
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct ValidatedJson<T: DeserializeOwned + Validate>(pub T);

#[async_trait]
impl<T, S> FromRequest<S> for ValidatedJson<T>
where
    T: DeserializeOwned + Validate,
    S: Send + Sync,
    Json<T>: FromRequest<S, Rejection = JsonRejection>,
{
    type Rejection = ValidatedJsonRejection;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let Json(json) = Json::<T>::from_request(req, state).await?;
        json.validate()?;
        Ok(Self(json))
    }
}
