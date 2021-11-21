
use std::rc::Rc;

use crate::common::*;
use crate::define_api;
use actix_web::{get, HttpResponse, Result};
use serde::Serialize;

use super::*;

define_api!(get("/identity"), Everyone);

async fn to_input(_: &mut utils::Context) -> Result<()> {
    Ok(())
}

#[derive(Serialize, Default)]
pub struct ResponseDto {
    #[serde(skip_serializing_if = "Option::is_none")]
    user: Option<UserDto>,
    #[serde(skip_serializing_if = "Option::is_none")]
    admin: Option<AdminDto>,
}

#[derive(Serialize)]
pub struct UserDto {
    pub name: Rc<str>,
    pub id: String,
}

#[derive(Serialize)]
pub struct AdminDto {
    pub id: String,
}

const STATUS: utils::Status = HttpResponse::Ok;

fn output_to_response(output: IdentityInfo) -> HttpResponse {
    let response_body = match output {
        IdentityInfo::User { id, name } => ResponseDto {
            user: Some(UserDto {
                name: name.into_rc_str(),
                id: utils::format_id(id.0),
            }),
            ..Default::default()
        },
        IdentityInfo::Admin { id } => ResponseDto {
            admin: Some(AdminDto {
                id: utils::format_id(id.0),
            }),
            ..Default::default()
        },
        IdentityInfo::Guest => Default::default(),
    };

    STATUS().json(response_body)
}
