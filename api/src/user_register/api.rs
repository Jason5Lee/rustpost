use actix_web::{post, web::Json, HttpResponse, Result};
use serde::{Deserialize, Serialize};

use crate::{common::*, define_token_api};

use super::Command;

define_token_api! {post("/register")}

#[derive(Deserialize)]
#[allow(non_snake_case)]
pub struct RequestDto {
    pub user_name: String,
    pub password: String,
}

async fn to_input(ctx: &mut utils::Context) -> Result<Command> {
    let req_body = ctx.to::<Json<RequestDto>>().await?.0;
    Ok(Command {
        user_name: UserName::try_new(req_body.user_name)
            .map_err(utils::Invalid::unprocessable_entity)?,
        password: Password::from_plain(req_body.password)
            .map_err(utils::Invalid::unprocessable_entity)?,
    })
}

const STATUS: utils::Status = HttpResponse::Created;

#[derive(Serialize)]
#[allow(non_snake_case)]
pub struct ResponseDto {
    pub token: String,
}

fn output_to_response(fresh_token: String) -> HttpResponse {
    STATUS().json(ResponseDto { token: fresh_token })
}
