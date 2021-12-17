use actix_web::{error::ErrorForbidden, Error};

pub fn user_name_or_password_incorrect() -> Error {
    ErrorForbidden("USER_NAME_OR_PASSWORD_INCORRECT")
}
