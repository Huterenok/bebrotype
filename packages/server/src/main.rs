mod controllers;
mod entities;
mod repositories;
mod services;
mod config;
mod utils;

use config::env::ENV;

use crate::controllers::create_router;
use crate::config::create_middleware;

#[tokio::main]
async fn main() {
		let stack_middleware = create_middleware();
    let app = create_router().layer(stack_middleware);

		tracing::info!("Server is running on {} url address", &ENV().await.url_address);
    axum::Server::bind(&ENV().await.url_address)
        .serve(app.into_make_service())
        .await
        .unwrap()
}
