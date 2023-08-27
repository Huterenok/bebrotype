use std::net::SocketAddr;
use dotenvy::var;
use tokio::sync::OnceCell;

pub struct Env {
    pub url_address: SocketAddr,
    pub database_url: String,
    pub secret_jwt_key: String,
}

pub fn load_env() -> Env {
    dotenvy::dotenv().expect("Couldn't load ENV variables");

    let url_address = std::env::var("PORT")
        .expect("Env variable PORT was not provided")
        .parse::<SocketAddr>()
        .unwrap();
    let database_url = var("DATABASE_URL").expect("Env variable DATABASE_URL was not provided");
    let secret_jwt_key =
        var("SECRET_JWT_KEY").expect("Env variable SECRET_JWT_KEY was not provided");

    Env {
        database_url,
        url_address,
        secret_jwt_key,
    }
}

static STATIC_ENV: OnceCell<Env> = OnceCell::const_new();
pub async fn ENV() -> &'static Env {
    STATIC_ENV.get_or_init(|| async { load_env() }).await
}
