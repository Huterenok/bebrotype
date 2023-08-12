use axum::response::IntoResponse;
use tokio::fs::OpenOptions;
use tokio::io::AsyncWriteExt;

use axum::extract::Multipart;

use crate::config::{Error, Result, CR};
use crate::entities::User;

use crate::controllers::users::dto::{
    AllUsersQuery, CreateUserDto, UpdateUserDto,
};

use crate::repositories::users::{create, get_all, get_by_email, get_by_id, update, update_favourites_texts};

use crate::utils::validate_image_file_format;

pub async fn create_user(mut dto: CreateUserDto) -> Result<User> {
    dto.password = CR().await.mc_encrypt(&dto.password);
    let res = create(dto).await?;
    Ok(res.into())
}

pub async fn get_user_by_id(id: i64) -> Result<User> {
    let res = get_by_id(id).await?;
    Ok(res.into())
}

pub async fn get_user_by_email(email: String) -> Result<User> {
    let res = get_by_email(email).await?;
    Ok(res.into())
}

pub async fn get_all_users(query: AllUsersQuery) -> Result<Vec<User>> {
    let limit = query.limit.unwrap_or(20);
    let res = get_all(limit).await?;
    Ok(res.into_iter().map(|user| user.into()).collect())
}

pub async fn update_user(mut multipart: Multipart, user: User) -> Result<User> {
    let mut user_near_address: Option<String> = None;
    let mut user_avatar: Option<String> = None;

    while let Some(field) = multipart.next_field().await.unwrap() {
        let name = field.name().unwrap().to_string();

        match name.as_str() {
            //TODO
            "avatar" => {
                validate_image_file_format(field.content_type().unwrap())?;
                let bytes = field.bytes().await.unwrap();
                let path = format!("static/user/avatar/{}.jpeg", user.id);
                let mut file = match OpenOptions::new()
                    .create(true)
                    .write(true)
                    .open(&path)
                    .await
                {
                    Ok(file) => file,
                    Err(_) => {
                        //TODO
                        tracing::error!("Error while opening file");
                        return Err(Error::InternalServerError.into_response());
                    }
                };
                file.write_all(&bytes).await.unwrap();
                user_avatar = Some(path)
            }
            "near_address" => match field.text().await {
                Ok(address) if address.len() > 0 => user_near_address = Some(address),
                //TODO
                Ok(_) => (),
                Err(_) => return Err(Error::BadOrganisedUserForm.into_response()),
            },
            _ => return Err(Error::BadOrganisedUserForm.into_response()),
        }
    }

    let dto = UpdateUserDto {
        near_address: user_near_address,
        avatar: user_avatar,
    };

    let res = update(user.id, dto).await?;
    Ok(res.into())
}

pub async fn update_favourites(id: i64, favourite_texts: Vec<Option<i64>>) -> Result<Vec<Option<i64>>> {
	let res = update_favourites_texts(id, favourite_texts).await?;
	Ok(res)
}