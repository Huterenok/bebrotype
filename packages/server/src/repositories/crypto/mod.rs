use axum::Json;
use hyper::StatusCode;

use lazy_static::lazy_static;

use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Validation};
use magic_crypt::{new_magic_crypt, MagicCrypt256, MagicCryptTrait};

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
    fn new() -> Self {
        let mc = new_magic_crypt!(&ENV.secret_magic_key, 256);
        let jwt = Jwt {
            decoding_key: DecodingKey::from_secret(ENV.secret_jwt_key.as_bytes()),
            encoding_key: EncodingKey::from_secret(ENV.secret_jwt_key.as_bytes()),
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
                Err((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(Error::InternalServerError),
                ))
            }
        }
    }
}

lazy_static! {
    pub static ref CR: Coder = Coder::new();
}
