use actix_web::error::ErrorNotFound;

pub fn creator_not_found() -> actix_web::Error {
    ErrorNotFound("CREATOR_NOT_FOUND")
}