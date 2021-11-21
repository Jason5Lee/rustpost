use actix_web::{
    error::{ErrorForbidden, ErrorNotFound},
    Error,
};

pub fn unauthorized() -> Error {
    ErrorForbidden("UNAUTHORIZED")
}

pub fn post_not_found() -> Error {
    ErrorNotFound("POST_NOT_FOUND")
}
