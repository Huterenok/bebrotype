use axum::{
    http::{HeaderMap, Request, StatusCode},
    middleware::Next,
    response::Response,
    Json,
};
use serde::{Deserialize, Serialize};

use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, Header};

use crate::{entities::User, services::user::get_user_by_email};

use crate::repositories::crypto::CR;
use crate::repositories::error::{Error, Result};

#[derive(Serialize, Deserialize)]
struct Claims {
    exp: usize,
    email: String,
}

pub async fn authentication_middleware<T>(
    headers: HeaderMap,
    mut request: Request<T>,
    next: Next<T>,
) -> Result<Response> {
    let header_token = extract_token(headers)?;

    let email = validate_token(&header_token)?;
    let user = get_user_by_email(email).await;

    match user {
        Ok(user) => {
            request.extensions_mut().insert(user);
        }
        Err(_) => return Err((StatusCode::UNAUTHORIZED, Json(Error::NotAuthorized))),
    };

    Ok(next.run(request).await)
}

pub fn generate_token(claims: &User) -> Result<String> {
    let now = Utc::now();
    let expires_at = Duration::days(7);
    let expires_at = now + expires_at;
    let exp = expires_at.timestamp() as usize;
    let claims = Claims {
        exp,
        email: claims.email.clone(),
    };

    match encode(&Header::default(), &claims, &CR.jwt.encoding_key) {
        Ok(token) => Ok(token),
        Err(err) => {
            eprintln!("->> Error generating token: {:?}", err);
            Err((StatusCode::UNAUTHORIZED, Json(Error::InternalServerError)))
        }
    }
}

pub fn validate_token(token: &str) -> Result<String> {
    decode::<Claims>(token, &CR.jwt.decoding_key, &CR.jwt.validation)
        .map_err(|error| match error.kind() {
            jsonwebtoken::errors::ErrorKind::InvalidToken
            | jsonwebtoken::errors::ErrorKind::InvalidSignature
            | jsonwebtoken::errors::ErrorKind::ExpiredSignature => {
                eprintln!("{:?}", error);
                (StatusCode::UNAUTHORIZED, Json(Error::NotAuthorized))
            }
            _ => (StatusCode::UNAUTHORIZED, Json(Error::NotAuthorized)),
        })
        .map(|data| data.claims.email)
}

pub fn extract_token(headers: HeaderMap) -> Result<String> {
    let mut header_token = if let Some(token) = headers.get("Authorization") {
        token.to_str().map_err(|error| {
            eprintln!("->> Error extracting token from headers: {:?}", error);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(Error::InternalServerError),
            )
        })?
    } else {
        return Err((StatusCode::UNAUTHORIZED, Json(Error::NotAuthorized)));
    };
    header_token = match header_token.split(" ").collect::<Vec<&str>>().get(1) {
        Some(&token) => token,
        None => return Err((StatusCode::UNAUTHORIZED, Json(Error::BadOrganisedToken))),
    };
		Ok(header_token.to_string())
}
