use actix_web::{error::ErrorForbidden, Error};

pub fn id_or_password_incorrect() -> Error {
    ErrorForbidden("ID_OR_PASSWORD_INCORRECT")
}
