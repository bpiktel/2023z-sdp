use axum::{
    async_trait,
    extract::{FromRef, FromRequestParts},
    http::request::Parts,
    RequestPartsExt,
};
use axum_extra::extract::CookieJar;
use hyper::StatusCode;
use jsonwebtoken::{decode, encode, Algorithm, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use tracing::{debug, error};
use uuid::Uuid;

use super::AuthKeys;

/// JWT claims
/// If retrieved successfuly, the client is authenticated
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Claims {
    pub exp: u64,
    pub nbf: u64,
    pub iat: u64,
    pub user_id: Uuid,
    pub username: String,
}

pub fn create_token(claims: Claims, key: &EncodingKey) -> Option<String> {
    encode(&Header::new(Algorithm::RS256), &claims, key)
        .map_err(|e| {
            error!({error = ?e}, "Error encoding JWT token");
        })
        .ok()
}

pub const JWT_TOKEN_COOKIE: &str = "jwt_token";

#[async_trait]
impl<S> FromRequestParts<S> for Claims
where
    AuthKeys: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = StatusCode;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let cookies = parts.extract::<CookieJar>().await.unwrap();
        let Some(cookie) = cookies.get(JWT_TOKEN_COOKIE) else {
            debug!("Missing JWT token cookie");
            return Err(StatusCode::UNAUTHORIZED);
        };

        let keys = AuthKeys::from_ref(state);

        let mut validation = Validation::new(Algorithm::RS256);
        validation.validate_nbf = true;
        let token_data = decode::<Claims>(cookie.value_trimmed(), &keys.decoding, &validation)
            .map_err(|e| {
                debug!({error = ?e}, "Error decoding JWT token");
                StatusCode::UNAUTHORIZED
            })?;

        Ok(token_data.claims)
    }
}

/// Optional JWT claims
/// Claims are available if retrieved succesfully
/// Retrieval never fails
pub struct OptClaims(Option<Claims>);

#[async_trait]
impl<S> FromRequestParts<S> for OptClaims
where
    AuthKeys: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = StatusCode;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let claims = Claims::from_request_parts(parts, state).await;
        Ok(Self(claims.ok()))
    }
}

impl OptClaims {
    pub fn logged_in(&self) -> bool {
        self.0.is_some()
    }
}
