use axum::Json;
use hyper::StatusCode;

use diesel_async::pooled_connection::deadpool::{Object, Pool};
use diesel_async::pooled_connection::AsyncDieselConnectionManager;
use diesel_async::AsyncPgConnection;

use async_once::AsyncOnce;
use lazy_static::lazy_static;

use super::env::ENV;
use crate::repositories::error::{Error, Result};

pub struct Connector {
    pub pool: Pool<AsyncPgConnection>,
}

impl Connector {
    pub async fn new() -> Self {
        let config =
            AsyncDieselConnectionManager::<diesel_async::AsyncPgConnection>::new(&ENV.database_url);
        let pool = Pool::builder(config).build().unwrap();
        Connector { pool }
    }

    pub async fn get_conn(&self) -> Result<Object<AsyncPgConnection>> {
        match self.pool.get().await {
            Ok(pool) => Ok(pool),
            Err(err) => {
                eprintln!("->> Error getting database connection: {:?}", err);
                Err((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(Error::InternalServerError),
                ))
            }
        }
    }
}

lazy_static! {
    pub static ref DB: AsyncOnce<Connector> = AsyncOnce::new(async { Connector::new().await });
}
