//TODO: WHOLE STRUCTURE OF APP_ERROR

use axum::response::Response;
use axum_error_macro::IntoResponse;
use serde::{Deserialize, Serialize};

pub type Result<T> = core::result::Result<T, Response>;

#[derive(Serialize, Deserialize, Debug, IntoResponse)]
pub enum Error {
    #[error(code = 500, msg = "Internal Server Error")]
    InternalServerError,
		#[error(code = 400, msg = "{}")]
    InvalidBody(String),
		
		#[error(code = 400, msg = "Token must match: Bearer token")]
    BadOrganisedToken,
		#[error(code = 400, msg = "Couldn't receive file in {} format")]
    WrongFileFormat(String),
		#[error(code = 400, msg = "User update form must contain: avatar | near_address")]
    BadOrganisedUserForm,

		#[error(code = 404, msg = "Couldn't find user with {} id")]
    UserByIdNotFound(i64),
		#[error(code = 404, msg = "Couldn't find user with {} email")]
    UserByEmailNotFound(String),
		#[error(code = 404, msg = "Couldn't find user with {} username")]
    UserByUsernameNotFound(String),
		#[error(code = 404, msg = "Couldn't find any users")]
    UsersNotFound,
		#[error(code = 400, msg = "User already exists")]
    UserAlreadyExist,
		#[error(code = 400, msg = "User with this email or username already exists")]
    NearUserAlreadyExist,
		
		#[error(code = 500, msg = "Couldn't create text")]
    TextCreationFail,
		#[error(code = 404, msg = "Couldn't find text with {} id")]
    TextByIdNotFound(i64),
		#[error(code = 404, msg = "Couldn't find text by {} user_id")]
    TextByUserIdNotFound(i64),
		#[error(code = 400, msg = "You don't have any liked texts")]
    DontHaveLikedTexts,

		#[error(code = 400, msg = "Wrong authentication credentials were provided")]
    AuthWrongCredentials,
		#[error(code = 401, msg = "You are not authorized")]
    NotAuthorized,
		#[error(code = 403, msg = "You are not allowed to do this")]
    NotAllowed,
}

