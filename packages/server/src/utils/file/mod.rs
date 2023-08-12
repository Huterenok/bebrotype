use axum::response::IntoResponse;

use crate::config::error::{Error, Result};

pub fn validate_image_file_format(content_type: &str) -> Result<()> {
    let file_format = content_type.split("/").collect::<Vec<&str>>()[1];
    for i in ["png", "jpeg", "jpg"].into_iter() {
        if i == file_format {
            return Ok(());
        }
    }
    return Err(Error::WrongFileFormat(file_format.to_owned()).into_response());
}
