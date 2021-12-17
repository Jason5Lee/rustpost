use actix_web::error::ErrorNotFound;

pub fn user_not_found() -> actix_web::Error {
    ErrorNotFound("USER_NOT_FOUND")
}
