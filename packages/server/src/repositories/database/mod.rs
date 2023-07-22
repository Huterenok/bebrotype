use async_once::AsyncOnce;
use diesel_async::pooled_connection::deadpool::Pool;
use diesel_async::pooled_connection::AsyncDieselConnectionManager;
use diesel_async::AsyncPgConnection;
use lazy_static::lazy_static;

use super::env::load_env;

pub struct Connector {
    pub pool: Pool<AsyncPgConnection>,
}

impl Connector {
    pub async fn new() -> Self {
        let env = load_env();
        let config =
            AsyncDieselConnectionManager::<diesel_async::AsyncPgConnection>::new(env.database_url);
        let pool = Pool::builder(config).build().unwrap();
        Connector { pool }
    }
}

lazy_static! {
    pub static ref DB: AsyncOnce<Connector> = AsyncOnce::new(async { Connector::new().await });
}
