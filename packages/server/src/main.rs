mod controllers;
mod entities;
mod repositories;
mod services;

use repositories::env::ENV;

use crate::controllers::create_router;

#[tokio::main]
async fn main() {
    let app = create_router();

    println!("->> LISTENING on {}\n", &ENV().await.url_address);
    axum::Server::bind(&ENV().await.url_address)
        .serve(app.into_make_service())
        .await
        .unwrap()
}
