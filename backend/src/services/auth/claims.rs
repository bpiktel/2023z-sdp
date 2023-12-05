use axum::{
    async_trait,
    extract::{FromRef, FromRequestParts},
    http::request::Parts,
    RequestPartsExt,
};
use axum_extra::{
    headers::{authorization::Bearer, Authorization},
    TypedHeader,
};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use tracing::error;
use uuid::Uuid;

use super::{error::AuthError, AuthKeys};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Claims {
    pub exp: u64,
    pub nbf: u64,
    pub iat: u64,
    pub user_id: Uuid,
    pub username: String,
}

impl Claims {
    pub fn try_into_token(self, key: &EncodingKey) -> Result<String, AuthError> {
        encode(&Header::new(Algorithm::RS256), &self, key).map_err(|error| {
            error!({error = ?error}, "Error encoding JWT token");
            AuthError::TokenCreation
        })
    }

    pub fn try_from_token(token: &str, key: &DecodingKey) -> Result<Self, AuthError> {
        let mut validation = Validation::new(Algorithm::RS256);
        validation.validate_nbf = true;
        let token_data = decode::<Claims>(token, key, &validation).map_err(|error| {
            error!({error = ?error}, "Error decoding JWT token");
            AuthError::InvalidToken
        })?;
        Ok(token_data.claims)
    }
}

#[async_trait]
impl<S> FromRequestParts<S> for Claims
where
    AuthKeys: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = AuthError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let TypedHeader(Authorization(bearer)) = parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await
            .map_err(|_| AuthError::InvalidToken)?;

        let keys = AuthKeys::from_ref(state);

        let token_data = decode::<Claims>(bearer.token(), &keys.decoding, &Validation::default())
            .map_err(|_| AuthError::InvalidToken)?;

        Ok(token_data.claims)
    }
}
