use actix_web::error::ErrorConflict;
use actix_web::Error;

pub fn duplicate_title() -> Error {
    ErrorConflict("DUPLICATE_TITLE")
}
