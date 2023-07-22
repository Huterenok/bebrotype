use axum::{Router, Json};
use axum::routing::post;

use crate::entities::User;
use crate::repositories::error::Result;

use super::user::{CreateUserRequest, create_user};

pub fn create_auth_service() -> Router {
	Router::new().route("/register", post(register))
}

async fn register(Json(register_request): Json<CreateUserRequest>) -> Result<Json<User>> {
	let user = match create_user(register_request).await {
			Ok(user) => user,
			Err(err) => return Err(err)
	};

	Ok(Json(user))
}

