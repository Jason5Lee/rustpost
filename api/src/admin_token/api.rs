use super::*;
use crate::common::*;
use crate::define_token_api;
use actix_web::HttpResponse;
use actix_web::{post, web::Json as BodyJson, Result};
use serde::Deserialize;
use serde::Serialize;

define_token_api!(post("/admin/token"));

#[derive(Deserialize)]
pub struct RequestDto {
    pub id: String,
    pub password: String,
}

async fn to_input(ctx: &mut utils::Context) -> Result<Query> {
    let req_body = ctx.to::<BodyJson<RequestDto>>().await?.0;

    Ok(Query {
        id: AdminId(utils::parse_id(&req_body.id)?),
        password: Password::from_plain(req_body.password)
            .map_err(utils::Invalid::unprocessable_entity)?,
    })
}

#[derive(Serialize)]
pub struct ResponseDto {
    token: String,
}

const STATUS: utils::Status = HttpResponse::Ok;

fn output_to_response(fresh_token: String) -> HttpResponse {
    STATUS().json(ResponseDto { token: fresh_token })
}
