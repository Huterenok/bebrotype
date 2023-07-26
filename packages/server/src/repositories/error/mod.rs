use axum::response::IntoResponse;
use axum::Json;
use hyper::StatusCode;
use serde::{Deserialize, Serialize};

pub type Result<T> = core::result::Result<T, (StatusCode, Json<Error>)>;

#[derive(Serialize, Deserialize, Debug)]
pub enum Error {
    InternalServerError,
    BadOrganisedToken,

    UserNotFound,
    UserAlreadyExist,

    AuthWrongCredentials,
    NotAuthorized,
}

impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let res = match self {
            Error::InternalServerError => "Server internal error",
            Error::BadOrganisedToken => "Token must match: Bearer token",

            Error::UserAlreadyExist => "User with this email or username already exists",
            Error::UserNotFound => "Couldn't find user",

            Error::NotAuthorized => "You are not authorized",
            Error::AuthWrongCredentials => "Wrong authentication credentials were provided",
        };
        write!(f, "{:?}", res)
    }
}

impl std::error::Error for Error {}

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        let mut response = StatusCode::INTERNAL_SERVER_ERROR.into_response();
        response.extensions_mut().insert(self);

        response
    }
}
