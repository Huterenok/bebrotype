use axum::response::IntoResponse;

use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Validation};
use magic_crypt::{new_magic_crypt, MagicCrypt256, MagicCryptTrait};
use tokio::sync::OnceCell;

use crate::repositories::env::ENV;
use crate::repositories::error::{Error, Result};

pub struct Jwt {
    pub decoding_key: DecodingKey,
    pub encoding_key: EncodingKey,
    pub validation: Validation,
}

pub struct Coder {
    pub mc: MagicCrypt256,
    pub jwt: Jwt,
}

impl Coder {
    async fn new() -> Self {
        let mc = new_magic_crypt!(&ENV().await.secret_magic_key, 256);
        let jwt = Jwt {
            decoding_key: DecodingKey::from_secret(ENV().await.secret_jwt_key.as_bytes()),
            encoding_key: EncodingKey::from_secret(ENV().await.secret_jwt_key.as_bytes()),
            validation: Validation::new(Algorithm::HS256),
        };
        Coder { mc, jwt }
    }

    pub fn mc_encrypt(&self, str: &str) -> String {
        self.mc.encrypt_str_to_base64(str)
    }

    pub fn mc_decrypt(&self, str: &str) -> Result<String> {
        match self.mc.decrypt_base64_to_string(str) {
            Ok(decrypted) => Ok(decrypted),
            Err(err) => {
                eprintln!("->> Error while mc_decrypting: {:?}", err);
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
