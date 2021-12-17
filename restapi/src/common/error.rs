use crate::common::utils::Invalid;
use actix_web::{
    error::{ErrorForbidden, ErrorUnauthorized},
    Error,
};
use actix_web::error::ErrorTooManyRequests;

pub fn unauthenticated() -> Error {
    ErrorUnauthorized("UNAUTHENTICATED")
}

pub fn auth_expired() -> Error {
    ErrorUnauthorized("AUTH_EXPIRED")
}

pub fn user_name_empty(value: String) -> Invalid<String> {
    Invalid::new(value, "USER_NAME_EMPTY")
}

pub fn user_name_too_short(value: String) -> Invalid<String> {
    Invalid::new(value, "USER_NAME_TOO_SHORT")
}

pub fn user_name_too_long(value: String) -> Invalid<String> {
    Invalid::new(value, "USER_NAME_TOO_LONG")
}

pub fn user_name_contains_illegal_character(value: String) -> Invalid<String> {
    Invalid::new(value, "USER_NAME_ILLEGAL")
}

pub fn title_empty(value: String) -> Invalid<String> {
    Invalid::new(value, "TITLE_EMPTY")
}

pub fn title_too_short(value: String) -> Invalid<String> {
    Invalid::new(value, "TITLE_TOO_SHORT")
}

pub fn title_too_long(value: String) -> Invalid<String> {
    Invalid::new(value, "TITLE_TOO_LONG")
}

pub fn password_empty(value: String) -> Invalid<String> {
    Invalid::new(value, "PASSWORD_EMPTY")
}
pub fn password_too_short(value: String) -> Invalid<String> {
    Invalid::new(value, "PASSWORD_TOO_SHORT")
}
pub fn password_too_long(value: String) -> Invalid<String> {
    Invalid::new(value, "PASSWORD_TOO_LONG")
}

pub fn invalid_url(value: String, reason: url::ParseError) -> Invalid<String> {
    Invalid::new(value, format!("URL_INVALID: {}", reason))
}

pub fn unauthorized() -> Error {
    ErrorForbidden("UNAUTHORIZED")
}
pub fn guest_only() -> Error {
    ErrorForbidden("GUEST_ONLY")
}

pub fn user_only() -> Error {
    ErrorForbidden("USER_ONLY")
}

pub fn too_many_requests() -> Error {
    ErrorTooManyRequests("TOO_MANY_REQUESTS")
}