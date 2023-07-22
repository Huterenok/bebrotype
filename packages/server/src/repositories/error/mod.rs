use axum::Json;
use axum::response::IntoResponse;
use hyper::StatusCode;
use serde::{Deserialize, Serialize};

pub type Result<T> = core::result::Result<T, (StatusCode, Json<Error>)>;

#[derive(Serialize, Deserialize, Debug)]
pub enum Error {
	UserNotFound,
	UserAlreadyExist
}

impl core::fmt::Display for Error {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
			write!(f, "{:?}", self)
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