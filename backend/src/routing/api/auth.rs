use axum::{
    extract::FromRef,
    extract::State,
    routing::{get, post},
    Json, Router,
};
use axum_extra::extract::{cookie::Cookie, CookieJar};
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
    database::{repositories::user::UserRepository, surreal::SurrealDb},
    util::{ResponseType, ValidatedJson},
};

pub fn auth_router<T>() -> Router<T>
where
    AuthKeys: FromRef<T>,
    SurrealDb: FromRef<T>,
    T: 'static + Send + Sync + Clone,
{
    Router::new()
        .route("/login", post(login))
        .route("/logout", post(logout))
        .route("/status", get(status))
}

#[derive(Debug, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct AuthorizeRequest {
    #[validate(length(min = 1, max = 63))]
    username: String,
    #[validate(length(min = 1, max = 63))]
    password: String,
}

async fn login(
    State(keys): State<AuthKeys>,
    user_repo: UserRepository,
    ValidatedJson(data): ValidatedJson<AuthorizeRequest>,
) -> ResponseType<CookieJar> {
    let Ok(is_admin) = user_repo
        .is_admin(&data.username, &data.password)
        .await
        .map_err(|e| error!({error = ?e}, "Error checking admin"))
    else {
        return ResponseType::Status(StatusCode::INTERNAL_SERVER_ERROR);
    };
    if !is_admin {
        return ResponseType::Status(StatusCode::BAD_REQUEST);
    }

    let now = Utc::now();
    let exp = now + Duration::days(1);
    let exp_ts = exp.timestamp() as u64;
    let now_ts = now.timestamp() as u64;
    let claims = Claims {
        exp: exp_ts,
        nbf: now_ts,
        iat: now_ts,
        user_id: Uuid::nil(),
        username: data.username,
    };
    let Some(token) = create_token(claims, &keys.encoding) else {
        return ResponseType::Status(StatusCode::INTERNAL_SERVER_ERROR);
    };
    let cookies = CookieJar::new();
    let mut cookie = Cookie::new(JWT_TOKEN_COOKIE, token);
    cookie.set_http_only(true);
    cookie.set_expires(None);
    ResponseType::Data(cookies.add(cookie))
}

async fn logout() -> CookieJar {
    let cookies = CookieJar::new();
    cookies.remove(Cookie::from(JWT_TOKEN_COOKIE))
}

async fn status(claims: Claims) -> Json<Claims> {
    Json(claims)
}
