mod entities;
mod repositories;
mod services;

use axum::Router;
use repositories::env::load_env;

use crate::services::{create_auth_service, create_user_service};

#[tokio::main]
async fn main() {
    let env = load_env();

    let app = Router::new()
        .nest("/user", create_user_service())
				.nest("/auth", create_auth_service());

    println!("->> LISTENING on {}\n", &env.url_address);
    axum::Server::bind(&env.url_address)
        .serve(app.into_make_service())
        .await
        .unwrap()
}
