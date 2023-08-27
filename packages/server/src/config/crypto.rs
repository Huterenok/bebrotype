use axum::response::IntoResponse;
use tokio::sync::OnceCell;

use bcrypt::{hash, verify};
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Validation};

use super::env::ENV;
use crate::common::{Error, Result};

pub struct Jwt {
    pub decoding_key: DecodingKey,
    pub encoding_key: EncodingKey,
    pub validation: Validation,
}

pub struct Coder {
    pub jwt: Jwt,
}

const DEFAULT_COST: u32 = 4;

impl Coder {
    async fn new() -> Self {
        let jwt = Jwt {
            decoding_key: DecodingKey::from_secret(ENV().await.secret_jwt_key.as_bytes()),
            encoding_key: EncodingKey::from_secret(ENV().await.secret_jwt_key.as_bytes()),
            validation: Validation::new(Algorithm::HS256),
        };
        Coder { jwt }
    }

    pub fn encrypt(str: &str) -> Result<String> {
        match hash(str, DEFAULT_COST) {
            Ok(hashed) => Ok(hashed),
            Err(err) => {
                tracing::error!("Error while encrypting: {}", err);
                Err(Error::InternalServerError.into_response())
            }
        }
    }

    pub fn decrypt(str: &str, hashed_part: &str) -> Result<bool> {
        match verify(str, hashed_part) {
            Ok(is_equal) => Ok(is_equal),
            Err(err) => {
                tracing::error!("Error while decrypting: {}", err);
                Err(Error::InternalServerError.into_response())
            }
        }
    }
}

static STATIC_CRYPTO: OnceCell<Coder> = OnceCell::const_new();
pub async fn CR() -> &'static Coder {
    STATIC_CRYPTO
        .get_or_init(|| async { Coder::new().await })
        .await
}
