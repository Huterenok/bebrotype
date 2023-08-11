use std::net::SocketAddr;

use dotenv::dotenv;
use tokio::sync::OnceCell;

pub struct Env {
    pub url_address: SocketAddr,
    pub database_url: String,
    pub secret_magic_key: String,
    pub secret_jwt_key: String,
}

pub fn load_env() -> Env {
    dotenv().ok();

    let url_address = std::env::var("PORT")
        .expect("Env variable PORT was not provided")
        .parse::<SocketAddr>()
        .unwrap();
    let database_url =
        std::env::var("DATABASE_URL").expect("Env variable DATABASE_URL was not provided");
    let secret_magic_key =
        std::env::var("SECRET_MAGIC_KEY").expect("Env variable SECRET_MAGIC_KEY was not provided");
    let secret_jwt_key =
        std::env::var("SECRET_JWT_KEY").expect("Env variable SECRET_JWT_KEY was not provided");

    Env {
        database_url,
        url_address,
        secret_magic_key,
        secret_jwt_key,
    }
}

static STATIC_ENV: OnceCell<Env> = OnceCell::const_new();
pub async fn ENV() -> &'static Env {
    STATIC_ENV.get_or_init(|| async { load_env() }).await
}
