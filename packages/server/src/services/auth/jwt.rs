use axum::{
    http::{HeaderMap, Request},
    middleware::Next,
    response::{IntoResponse, Response},
};
use serde::{Deserialize, Serialize};

use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, Header};

use crate::{
    common::{Error, Result},
    config::CR,
    entities::User,
};

use super::get_user_by_email;

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

    let email = validate_token(&header_token).await?;
    let user = get_user_by_email(email).await;

    match user {
        Ok(user) => {
            request.extensions_mut().insert(user);
        }
        Err(_) => return Err(Error::NotAuthorized.into_response()),
    };

    Ok(next.run(request).await)
}

pub async fn generate_token(claims: &User) -> Result<String> {
    let now = Utc::now();
    let expires_at = Duration::days(7);
    let expires_at = now + expires_at;
    let exp = expires_at.timestamp() as usize;
    let claims = Claims {
        exp,
        email: claims.email.clone(),
    };

    match encode(&Header::default(), &claims, &CR().await.jwt.encoding_key) {
        Ok(token) => Ok(token),
        Err(_) => {
            tracing::error!("Error while encoding token");
            Err(Error::InternalServerError.into_response())
        }
    }
}

pub async fn validate_token(token: &str) -> Result<String> {
    decode::<Claims>(
        token,
        &CR().await.jwt.decoding_key,
        &CR().await.jwt.validation,
    )
    .map_err(|error| match error.kind() {
        jsonwebtoken::errors::ErrorKind::InvalidToken
        | jsonwebtoken::errors::ErrorKind::InvalidSignature
        | jsonwebtoken::errors::ErrorKind::ExpiredSignature => Error::NotAuthorized.into_response(),
        _ => Error::NotAuthorized.into_response(),
    })
    .map(|data| data.claims.email)
}

pub fn extract_token(headers: HeaderMap) -> Result<String> {
    let mut header_token = if let Some(token) = headers.get("Authorization") {
        match token.to_str() {
            Ok(token) => token,
            Err(_) => return Err(Error::BadOrganisedToken.into_response()),
        }
    } else {
        return Err(Error::NotAuthorized.into_response());
    };
    header_token = match header_token.split(" ").collect::<Vec<&str>>().get(1) {
        Some(&token) => token,
        None => return Err(Error::BadOrganisedToken.into_response()),
    };
    Ok(header_token.to_string())
}
