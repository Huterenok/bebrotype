use axum::response::{IntoResponse, Response};
use axum::{async_trait, extract::FromRequest, Json, RequestExt};
use hyper::Request;
use validator::Validate;

use super::Error;

pub struct ValidatedJson<J>(pub J);

#[async_trait]
impl<S, B, J> FromRequest<S, B> for ValidatedJson<J>
where
    B: Send + 'static,
    S: Send + Sync,
    J: Validate + 'static,
    Json<J>: FromRequest<(), B>,
{
    type Rejection = Response;

    async fn from_request(req: Request<B>, _state: &S) -> Result<Self, Self::Rejection> {
        let Json(data) = req
            .extract::<Json<J>, _>()
            .await
            .map_err(|_| Error::InvalidBody("Invalid data".to_string()).into_response())?;

        if let Err(e) = data.validate() {
						tracing::error!("{:?}", e);
            let msg: String = e
                .field_errors()
                .iter()
                .flat_map(|err_map| err_map.1.iter())
                .filter_map(|err| err.message.clone())
                .map(|msg_cow| {
									tracing::error!("{}", msg_cow.to_string());
									msg_cow.to_string()
								})
                .collect::<Vec<String>>()
                .join("\n");

            return Err(Error::InvalidBody(msg).into_response());
        }
        Ok(Self(data))
    }
}
