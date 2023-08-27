use std::collections::HashMap;

use axum::response::{IntoResponse, Redirect};
use oauth2::{
    reqwest::http_client, AuthorizationCode, CsrfToken, PkceCodeChallenge, PkceCodeVerifier, Scope,
    TokenResponse,
};

use reqwest;
use serde_json::Value;

use crate::{
    common::{Error, Result},
    config::oauth::{OAuthData, OAuthSharedState},
    controllers::users::dto::CreateUserDto,
    services::users::{create_user, get_user_by_email},
};

use super::jwt::generate_token;

pub async fn oauth_google(
    mut params: HashMap<String, String>,
    state: OAuthSharedState,
) -> Result<Redirect> {
    //TODO
    let mut state = state.lock().await;

    //TODO: validate
    let return_url = params
        .remove("return_url")
        .unwrap_or_else(|| "/".to_string());

    let (pkce_code_challenge, pkce_code_verifier) = PkceCodeChallenge::new_random_sha256();

    let (authorize_url, csrf_state) = state
        .client
        .authorize_url(CsrfToken::new_random)
        .add_scopes(vec![
            Scope::new("https://www.googleapis.com/auth/userinfo.email".to_string()),
            Scope::new("https://www.googleapis.com/auth/userinfo.profile".to_string()),
        ])
        .set_pkce_challenge(pkce_code_challenge)
        .url();

    state.save_verifier(
        csrf_state.secret().to_owned(),
        OAuthData::new(pkce_code_verifier.secret().to_owned(), return_url),
    );

    Ok(Redirect::to(authorize_url.as_str()))
}

//TODO: better response with data without token in query
pub async fn oauth_google_return(
    mut params: HashMap<String, String>,
    state: OAuthSharedState,
) -> Result<Redirect> {
    //TODO
    let mut state = state.lock().await;
    //TODO
    let client = state.client.clone();

    let csrf_state = match params.remove("state") {
        Some(token) => CsrfToken::new(token),
        //TODO
        None => {
            tracing::error!("Wrong CSRF token");
            return Err(Error::InternalServerError.into_response());
        }
    };
    let code = match params.remove("code") {
        Some(token) => AuthorizationCode::new(token),
        //TODO
        None => {
            tracing::error!("Wrong authorization code");
            return Err(Error::InternalServerError.into_response());
        }
    };

    let oauth_data = state.delete_verifier(csrf_state.secret().to_owned())?;
    let pkce_code_verifier = PkceCodeVerifier::new(oauth_data.pkc_code);

    let token_response = match tokio::task::spawn_blocking(move || {
        client
            .exchange_code(code)
            .set_pkce_verifier(pkce_code_verifier)
            .request(http_client)
    })
    .await
    {
        Ok(token) => token,
        Err(_) => {
            tracing::error!("Error while requesting token");
            return Err(Error::InternalServerError.into_response());
        }
    }
    //TODO
    .unwrap();
    let access_token = token_response.access_token().secret();

    let url = format!(
        "https://www.googleapis.com/oauth2/v2/userinfo?oauth_token={}",
        access_token
    );
    let body = reqwest::get(url).await.unwrap().text().await.unwrap();
    let data = retrieve_data_from_access_token(body)?;

    let user = match get_user_by_email(data.email.clone()).await {
        Ok(user) => user,
        Err(_) => {
            create_user(CreateUserDto {
                username: data.username,
                email: data.email,
                password: data.password,
                avatar: data.avatar,
            })
            .await?
        }
    };

    let jwt_token = generate_token(&user).await?;
    // let headers = axum::response::AppendHeaders([(axum::http::header::AUTHORIZATION, jwt_token)]);

    Ok(Redirect::to(&format!(
        "{}?token={}",
        oauth_data.return_url, jwt_token
    )))
}

struct RetrievedData {
    email: String,
    //TODO: think about storing access token in db
    password: String,
    username: String,
    avatar: Option<String>,
}

fn retrieve_data_from_access_token(body: String) -> Result<RetrievedData> {
    let body: Value = serde_json::from_str(body.as_str()).unwrap();
    let email = match body.get("email") {
        Some(data) => data.to_string(),
        None => return Err(Error::OAuthWrongCredentials.into_response()),
    };
    match body.get("verified_email") {
        Some(data) => match data.as_bool().unwrap() {
            true => (),
            false => return Err(Error::OAuthEmailNotVerified.into_response()),
        },
        None => return Err(Error::OAuthWrongCredentials.into_response()),
    };
    let username = match body.get("name") {
        Some(data) => data.to_string(),
        None => return Err(Error::OAuthWrongCredentials.into_response()),
    };
    let avatar = match body.get("picture") {
        Some(data) => Some(data.to_string()),
        None => None,
    };

    Ok(RetrievedData {
        email,
        //TODO: think about storing access token in db
        password: "".into(),
        username,
        avatar,
    })
}
