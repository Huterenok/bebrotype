pub mod jwt;

use axum::response::IntoResponse;

use crate::config::{Error, Result, Coder};

use crate::controllers::auth::dto::{AuthPayload, LoginDto};
use crate::controllers::users::dto::CreateUserDto;

use self::jwt::generate_token;
use super::users::{create_user, get_user_by_email};

pub async fn register(register_request: CreateUserDto) -> Result<AuthPayload> {
    let user = create_user(register_request).await?;

    let token = generate_token(&user).await?;

    Ok(AuthPayload::new(token, user.into()))
}

pub async fn login(login_body: LoginDto) -> Result<AuthPayload> {
    let user = get_user_by_email(login_body.email).await?;

    if !Coder::decrypt(&login_body.password, &user.password)? {
        return Err(Error::AuthWrongCredentials.into_response());
    }

    let user = user.into();
    let token = generate_token(&user).await?;

    Ok(AuthPayload::new(token, user.into()))
}
