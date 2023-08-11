use axum::response::IntoResponse;
use tokio::fs::OpenOptions;
use tokio::io::AsyncWriteExt;

use axum::extract::Multipart;

use diesel::prelude::*;
use diesel_async::RunQueryDsl;

use crate::entities::schema::users::{
    dsl::users as users_table, email as email_column, id as id_column,
};
use crate::entities::User;

use crate::controllers::users::dto::{AllUsersQuery, CreateUserDto, UpdateUserDto};

use crate::repositories::crypto::CR;
use crate::repositories::database::DB;
use crate::repositories::error::{Error, Result};
use crate::repositories::model::file::validate_image_file_format;

pub async fn get_user_by_id(user_id: i32) -> Result<User> {
    let mut conn = DB().await.get_conn().await?;

    let result = users_table
        .find(user_id)
        .select(User::as_select())
        .first(&mut conn)
        .await;

    match result {
        Ok(user) => Ok(user),
        Err(_) => Err(Error::UserByIdNotFound(user_id).into_response()),
    }
}

pub async fn get_user_by_email(user_email: String) -> Result<User> {
    let mut conn = DB().await.get_conn().await?;

    let result = users_table
        .filter(email_column.eq(&user_email))
        .select(User::as_select())
        .first(&mut conn)
        .await;

    match result {
        Ok(user) => Ok(user),
        Err(_) => Err(Error::UserByEmailNotFound(user_email).into_response()),
    }
}

pub async fn get_all_users(query: AllUsersQuery) -> Result<Vec<User>> {
    let mut conn = DB().await.get_conn().await?;

    let result = users_table
        .limit(query.limit.unwrap_or(20) as i64)
        .select(User::as_select())
        .load(&mut conn)
        .await;

    match result {
        Ok(user) => Ok(user),
        Err(_) => Err(Error::UsersNotFound.into_response()),
    }
}

pub async fn create_user(mut dto: CreateUserDto) -> Result<User> {
    let mut conn = DB().await.get_conn().await?;

    dto.password = CR().await.mc_encrypt(&dto.password);
    let result = diesel::insert_into(users_table)
        .values(&dto)
        .returning(User::as_returning())
        .get_result(&mut conn)
        .await;

    match result {
        Ok(user) => Ok(user),
        Err(_) => Err(Error::UserAlreadyExist.into_response()),
    }
}

pub async fn update_user(mut multipart: Multipart, user: User) -> Result<User> {
    let mut conn = DB().await.get_conn().await?;

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
                    Err(err) => {
                        eprintln!("{err}");
                        return Err(Error::InternalServerError.into_response());
                    }
                };
                file.write_all(&bytes).await.unwrap();
                user_avatar = Some(path)
            }
            "near_address" => match field.text().await {
                Ok(address) if address.len() > 0 => user_near_address = Some(address),
                //TODO
                Ok(_) => user_near_address = None,
                Err(_) => return Err(Error::BadOrganisedUserForm.into_response()),
            },
            _ => return Err(Error::BadOrganisedUserForm.into_response()),
        }
    }

    let dto = UpdateUserDto {
        avatar: user_avatar,
        near_address: user_near_address,
    };

    let res = diesel::update(users_table.filter(id_column.eq(user.id)))
        .set(dto)
        .get_result::<User>(&mut conn)
        .await;

    match res {
        Ok(user) => Ok(user.into()),
        Err(err) => {
            eprintln!("{err}");
            Err(Error::UserAlreadyExist.into_response())
        }
    }
}
