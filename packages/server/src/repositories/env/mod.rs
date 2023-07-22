use std::net::SocketAddr;

use dotenv::dotenv;
pub struct Env {
    pub url_address: SocketAddr,
    pub database_url: String,
}

pub fn load_env() -> Env {
    dotenv().ok();

    let url_address = std::env::var("PORT")
        .expect("Env variable PORT was not provided")
        .parse::<SocketAddr>()
				.unwrap();
    let database_url =
        std::env::var("DATABASE_URL").expect("Env variable DATABASE_URL was not provided");

    Env { database_url, url_address }
}
