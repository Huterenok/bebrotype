use dotenvy::var;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;

use axum::response::IntoResponse;
use oauth2::basic::BasicClient;
use oauth2::{AuthUrl, ClientId, ClientSecret, RedirectUrl, RevocationUrl, TokenUrl};

use crate::common::{Error, Result};

pub type OAuthSharedState = Arc<Mutex<OAuthState>>;

#[derive(Debug)]
pub struct OAuthData {
	pub pkc_code: String,
	pub return_url: String,
}

pub struct OAuthState {
    pub client: BasicClient,
    pub oauth_data: HashMap<String, OAuthData>,
}

//TODO: think about env
impl OAuthState {
    pub fn new() -> Self {
        let auth_url = AuthUrl::new("https://accounts.google.com/o/oauth2/v2/auth".into())
            .expect("Wrong AuthUrl was provided");
        let token_url = TokenUrl::new("https://www.googleapis.com/oauth2/v3/token".to_string())
            .expect("Wrong TokenUrl was provided");
        let google_client_id = ClientId::new(
            var("GOOGLE_CLIENT_ID").expect("Env variable GOOGLE_CLIENT_ID was not provided"),
        );
        let google_client_secret = ClientSecret::new(
            var("GOOGLE_CLIENT_SECRET")
                .expect("Env variable GOOGLE_CLIENT_SECRET was not provided"),
        );

        //TODO: https
        let protocol = "http";

        let redirect_url = format!(
            //TODO
            "{}://localhost:3001/api/auth/google-oauth-return",
            protocol,
            // ENV().await.url_address
        );

        let client = BasicClient::new(
            google_client_id,
            Some(google_client_secret),
            auth_url,
            Some(token_url),
        )
        .set_redirect_uri(RedirectUrl::new(redirect_url).expect("Wrong redirect_url was provided"))
        .set_revocation_uri(
            RevocationUrl::new("https://oauth2.googleapis.com/revoke".to_string())
                .expect("RevocationUrl was provided"),
        );
        Self {
            client,
            oauth_data: HashMap::new(),
        }
    }

    pub fn save_verifier(&mut self, csrf_token: String, data: OAuthData) {
        self.oauth_data.insert(csrf_token, data);
    }

		//TODO
    pub fn delete_verifier(&mut self, csrf_token: String) -> Result<OAuthData> {
        match self.oauth_data.remove(&csrf_token) {
            Some(data) => Ok(data),
            None => {
                tracing::error!("Cant find {} csrf token", csrf_token);
                Err(Error::InternalServerError.into_response())
            }
        }
    }
}

impl OAuthData {
    pub fn new(pkc_code: String, return_url: String) -> Self {
        Self {
            pkc_code,
            return_url,
        }
    }
}
