use axum::{extract::State, Json};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use tracing::error;
use uuid::Uuid;
use validator::Validate;

use crate::services::{
    auth::{claims::Claims, error::AuthError, AuthKeys},
    database::{repositories::user::UserRepository, surreal::SurrealDb},
    util::ValidatedJson,
};

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

pub async fn login(
    State(keys): State<AuthKeys>,
    user_repo: UserRepository,
    ValidatedJson(data): ValidatedJson<AuthorizeRequest>,
) -> Result<Json<AuthorizeResponse>, AuthError> {
    //user_repo.is_admin(&data.username, &data.password)

    let now = Utc::now().timestamp() as u64;
    let claims = Claims {
        exp: now + 60 * 60 * 24,
        nbf: now,
        iat: now,
        user_id: Uuid::nil(),
        username: data.username,
    };
    let token = claims.try_into_token(&keys.encoding)?;
    Ok(Json(AuthorizeResponse::new(token)))
}
