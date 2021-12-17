use actix_web::{error::ErrorConflict, Error};

pub fn user_name_already_exists() -> Error {
    ErrorConflict("USER_NAME_ALREADY_EXISTS")
}
