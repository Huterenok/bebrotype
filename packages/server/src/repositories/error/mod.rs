//TODO: WHOLE STRUCTURE OF APP_ERROR

use axum::response::{IntoResponse, Response};
use hyper::StatusCode;
use serde::{Deserialize, Serialize};

pub type Result<T> = core::result::Result<T, Response>;

#[derive(Serialize, Deserialize, Debug)]
pub enum Error {
    InternalServerError,

    BadOrganisedToken,
    WrongFileFormat(String),
    BadOrganisedUserForm,

    UserByIdNotFound(i32),
    UserByEmailNotFound(String),
    UserByUsernameNotFound(String),
    //TODO
    UserAlreadyExist,
    UsersNotFound,

    TextCreationFail,
    TextByIdNotFound(i32),
    TextByUserIdNotFound(i32),

    AuthWrongCredentials,
    NotAuthorized,
    NotAllowed,
}

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        let error = match self {
            Error::InternalServerError => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Server internal error".to_string(),
            ),

            Error::BadOrganisedToken => (
                StatusCode::BAD_REQUEST,
                "Token must match: Bearer token".to_string(),
            ),
            Error::WrongFileFormat(format) => (
                StatusCode::BAD_REQUEST,
                format!("Couldn't receive file in {} format", format),
            ),

            Error::UserAlreadyExist => (
                StatusCode::BAD_REQUEST,
                "User with this email or username already exists".to_string(),
            ),
            Error::UserByIdNotFound(id) => (
                StatusCode::NOT_FOUND,
                format!("Couldn't find user with {} id", id),
            ),
            Error::UserByEmailNotFound(email) => (
                StatusCode::NOT_FOUND,
                format!("Couldn't find user with {} email", email),
            ),
            Error::UserByUsernameNotFound(username) => (
                StatusCode::NOT_FOUND,
                format!("Couldn't find user with {} username", username),
            ),
            Error::UsersNotFound => (StatusCode::NOT_FOUND, "Couldn't find any users".to_string()),

            Error::TextByIdNotFound(id) => (
                StatusCode::NOT_FOUND,
                format!("Couldn't find text with {} id", id),
            ),
            Error::TextByUserIdNotFound(id) => (
                StatusCode::NOT_FOUND,
                format!("Couldn't find text by {} user_id", id),
            ),
            Error::TextCreationFail => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Couldn't create text".to_string(),
            ),

            Error::NotAuthorized => (
                StatusCode::UNAUTHORIZED,
                "You are not authorized".to_string(),
            ),
            Error::AuthWrongCredentials => (
                StatusCode::BAD_REQUEST,
                "Wrong authentication credentials were provided".to_string(),
            ),
            Error::NotAllowed => (
                StatusCode::METHOD_NOT_ALLOWED,
                "You are not allowed to do this".to_string(),
            ),
            Error::BadOrganisedUserForm => (
                StatusCode::BAD_REQUEST,
                "User update form must contain: avatar | near_address".to_string(),
            ),
        };
        error.into_response()
    }
}
