use actix_web::{
    error::{ErrorForbidden, ErrorNotFound, ErrorUnprocessableEntity},
    Error,
};

pub fn post_not_found() -> Error {
    ErrorNotFound("POST_NOT_FOUND")
}
pub fn not_creator() -> Error {
    ErrorForbidden("NOT_CREATOR")
}
pub fn content_type_diff() -> Error {
    ErrorUnprocessableEntity("CONTENT_TYPE_DIFF")
}
