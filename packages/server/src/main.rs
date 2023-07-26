mod controllers;
mod entities;
mod repositories;
mod services;

use axum::Router;

use crate::controllers::{create_auth_routes, create_user_routes};

use repositories::env::ENV;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .nest("/user", create_user_routes())
        .nest("/auth", create_auth_routes());

    println!("->> LISTENING on {}\n", &ENV.url_address);
    axum::Server::bind(&ENV.url_address)
        .serve(app.into_make_service())
        .await
        .unwrap()
}
