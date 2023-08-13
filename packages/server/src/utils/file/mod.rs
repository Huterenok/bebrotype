use axum::response::IntoResponse;

use crate::config::error::{Error, Result};

const FORMATS: [&str; 3] = ["png", "jpeg", "jpg"];

pub fn validate_image_file_format(content_type: &str) -> Result<()> {
    let file_format = content_type.split("/").nth(1).unwrap();
    for i in FORMATS.into_iter() {
        if i == file_format {
            return Ok(());
        }
    }
    return Err(Error::WrongFileFormat(file_format.to_owned()).into_response());
}
