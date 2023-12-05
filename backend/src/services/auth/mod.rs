pub mod claims;
pub mod error;

use std::{error::Error, fs::read, path::PathBuf};

use jsonwebtoken::{DecodingKey, EncodingKey};
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct AuthKeysConfig {
    pub encoding: PathBuf,
    pub decoding: PathBuf,
}

#[derive(Clone)]
pub struct AuthKeys {
    pub encoding: EncodingKey,
    pub decoding: DecodingKey,
}

impl TryFrom<&AuthKeysConfig> for AuthKeys {
    type Error = Box<dyn Error>;

    fn try_from(value: &AuthKeysConfig) -> Result<Self, Self::Error> {
        let encoding = EncodingKey::from_rsa_pem(&read(&value.encoding)?)?;
        let decoding = DecodingKey::from_rsa_pem(&read(&value.decoding)?)?;
        Ok(Self { encoding, decoding })
    }
}
