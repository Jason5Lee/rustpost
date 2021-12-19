use actix_web::error::ErrorBadRequest;
use actix_web::Result;
use std::fmt::Debug;
use std::rc::Rc;

use self::utils::Invalid;

mod api;
pub mod api_common {
    pub use super::api::*;
}
mod error;
pub mod error_common {
    pub use super::error::*;
}
pub mod db;
pub mod utils;

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Copy, Clone)]
pub struct Time {
    pub utc: u64,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Identity {
    User(UserId),
    Admin(AdminId),
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct UserId(pub u64);

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct AdminId(pub u64);

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct UserName(Rc<str>);

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct User {
    pub id: UserId,
    pub name: UserName,
    pub creation: Time,
}

#[derive(PartialEq, Eq, Clone)]
pub struct Password {
    plain: String,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct LengthLimit(u32);

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct PostId(pub u64);

#[derive(PartialEq, Eq, Clone)]
pub struct Title(String);

#[derive(PartialEq, Eq, Clone)]
pub enum PostContent {
    Post(String),
    Url(url::Url),
}

#[derive(PartialEq, Eq, Clone, Copy)]
pub struct Size(u32);

impl UserName {
    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn into_rc_str(self) -> Rc<str> {
        self.0
    }

    fn is_legal_character(ch: char) -> bool {
        ch.is_ascii_alphanumeric() || ch == '_' || ch == '-'
    }

    pub fn try_new(input: String) -> Result<UserName, Invalid<String>> {
        if input.is_empty() {
            Err(error::user_name_empty(input))
        } else if input.len() < 3 {
            Err(error::user_name_too_short(input))
        } else if input.len() > 20 {
            Err(error::user_name_too_long(input))
        } else if !input.chars().all(UserName::is_legal_character) {
            Err(error::user_name_contains_illegal_character(input))
        } else {
            Ok(UserName(input.into()))
        }
    }
}

impl Title {
    pub fn try_new(input: String) -> Result<Title, Invalid<String>> {
        if input.is_empty() {
            Err(error::title_empty(input))
        } else if input.len() < 3 {
            Err(error::title_too_short(input))
        } else if input.len() > 171 {
            Err(error::title_too_long(input))
        } else {
            Ok(Title(input))
        }
    }
    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn into_string(self) -> String {
        self.0
    }
}

impl PostContent {
    pub fn parse_url(url: String) -> Result<PostContent, Invalid<String>> {
        url::Url::parse(&url)
            .map(PostContent::Url)
            .map_err(|reason| error::invalid_url(url, reason))
    }
}

impl Time {
    pub fn now() -> Time {
        Time {
            utc: utils::current_timestamp(),
        }
    }
}

impl Password {
    pub fn to_encrypted(&self, encryptor: &utils::Encryptor) -> Result<String> {
        encryptor.encrypt(&self.plain)
    }

    pub fn verify(&self, encrypted: &str) -> Result<bool> {
        utils::Encryptor::verify(&self.plain, encrypted)
    }

    pub fn from_plain(value: String) -> Result<Password, Invalid<String>> {
        if value.is_empty() {
            Err(error::password_empty(value))
        } else if value.len() < 5 {
            Err(error::password_too_short(value))
        } else if value.len() > 72 {
            Err(error::password_too_long(value))
        } else {
            Ok(Password { plain: value })
        }
    }
}

const DEFAULT_SIZE: u32 = 20;
const MAX_SIZE: u32 = 500;

impl Size {
    pub fn to_u32(self) -> u32 {
        self.0
    }
    pub fn try_new(s: Option<u32>) -> Result<Size> {
        match s {
            None => Ok(Size(DEFAULT_SIZE)),
            Some(size) => {
                if size == 0 {
                    Err(ErrorBadRequest("EMPTY_SIZE"))
                } else if size > MAX_SIZE {
                    Ok(Size(MAX_SIZE))
                } else {
                    Ok(Size(size))
                }
            }
        }
    }
}
