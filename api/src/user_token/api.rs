use super::*;
use crate::{
    common::utils::{Context, Status},
    define_token_api,
};
use actix_web::Result;
use actix_web::{post, web::Json, HttpResponse};
use serde::{Deserialize, Serialize};

define_token_api! {post("/token")}

#[derive(Deserialize)]
#[allow(non_snake_case)]
pub struct RequestDto {
    pub userName: String,
    pub password: String,
}

async fn to_input(ctx: &mut Context) -> Result<Query> {
    let req_body = ctx.to::<Json<RequestDto>>().await?.0;
    Ok(Query {
        user_name: UserName::try_new(req_body.userName)
            .map_err(utils::Invalid::unprocessable_entity)?,
        password: Password::from_plain(req_body.password)
            .map_err(utils::Invalid::unprocessable_entity)?,
    })
}

const STATUS: Status = HttpResponse::Created;

#[derive(Serialize)]
#[allow(non_snake_case)]
pub struct ResponseDto {
    pub token: String,
}

fn output_to_response(fresh_token: String) -> HttpResponse {
    STATUS().json(ResponseDto { token: fresh_token })
}
