use actix_web::{error::ErrorNotFound, Error};

pub fn post_not_found() -> Error {
    ErrorNotFound("POST_NOT_FOUND")
}
