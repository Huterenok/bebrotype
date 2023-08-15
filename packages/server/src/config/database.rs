use axum::response::IntoResponse;

use diesel_async::pooled_connection::deadpool::{Object, Pool};
use diesel_async::pooled_connection::AsyncDieselConnectionManager;
use diesel_async::AsyncPgConnection;

use tokio::sync::OnceCell;

use super::env::ENV;
use super::error::{Error, Result};

pub struct Connector {
    pub pool: Pool<AsyncPgConnection>,
}

impl Connector {
    pub async fn new() -> Self {
        let config = AsyncDieselConnectionManager::<diesel_async::AsyncPgConnection>::new(
            &ENV().await.database_url,
        );
        let pool = Pool::builder(config).build().unwrap();
        Connector { pool }
    }

    pub async fn get_conn(&self) -> Result<Object<AsyncPgConnection>> {
        match self.pool.get().await {
            Ok(pool) => Ok(pool),
            Err(_) => {
                //TODO
                tracing::error!("Error while getting connection");
                Err(Error::InternalServerError.into_response())
            }
        }
    }
}

static STATIC_DB: OnceCell<Connector> = OnceCell::const_new();
pub async fn DB() -> &'static Connector {
    STATIC_DB
        .get_or_init(|| async { Connector::new().await })
        .await
}
