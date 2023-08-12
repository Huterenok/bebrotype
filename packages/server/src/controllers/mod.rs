pub mod auth;
pub mod texts;
pub mod users;

use axum::{self, Router};

use tower_http::services::ServeDir;

use auth::create_auth_routes;
use texts::{create_private_text_routes, create_public_text_routes};
use users::{create_private_user_routes, create_public_user_routes};

use crate::services::auth::jwt::authentication_middleware;

pub fn create_router() -> Router {
    let public_routes = Router::new()
        .merge(create_auth_routes())
        .merge(create_public_user_routes())
        .merge(create_public_text_routes());

    let private_routes = Router::new()
        .merge(create_private_user_routes())
        .merge(create_private_text_routes())
        .layer(axum::middleware::from_fn(authentication_middleware));


    let app = Router::new()
        .merge(public_routes)
        .merge(private_routes)
        .nest_service("/static", ServeDir::new("static"));

    Router::new().nest("/api", app)
}
