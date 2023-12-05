use axum::{debug_handler, extract::State, http::StatusCode};
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::services::{auth::AuthKeys, util::ValidatedJson};

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AuthorizeResponse {
    token: String,
}

impl AuthorizeResponse {
    fn new(token: String) -> Self {
        Self { token }
    }
}

#[derive(Debug, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct AuthorizeRequest {
    #[validate(length(min = 1, max = 63))]
    username: String,
    #[validate(length(min = 1, max = 63))]
    password: String,
}

#[debug_handler]
pub async fn login(
    State(_keys): State<AuthKeys>,
    ValidatedJson(data): ValidatedJson<AuthorizeRequest>,
) -> StatusCode {
    dbg!(data); // WE LEAKIN THE ADMIN LOGIN DATA MUAHAHAHA
    StatusCode::OK
}
