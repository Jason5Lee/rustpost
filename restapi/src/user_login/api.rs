use super::*;
use crate::common::utils;
use actix_web::Result;
use actix_web::{error::ErrorUnprocessableEntity, post, web::Json as BodyJson, HttpResponse};
use serde::{Deserialize, Serialize};

#[post("/login")]
pub async fn api(mut ctx: utils::Context) -> Result<HttpResponse> {
    #[derive(Deserialize)]
    #[allow(non_snake_case)]
    pub struct RequestDto {
        pub userName: String,
        pub password: String,
    }
    let req_body = ctx.to::<BodyJson<RequestDto>>().await?.0;
    let input = Query {
        user_name: UserName::try_new(req_body.userName)
            .map_err(|err| ErrorUnprocessableEntity(err.body))?,
        password: Password::from_plain(req_body.password)
            .map_err(|err| ErrorUnprocessableEntity(err.body))?,
    };
    let output = super::Steps::from_ctx(&ctx).workflow(input).await?;
    Ok(HttpResponse::Ok().json({
        #[derive(Serialize)]
        #[allow(non_snake_case)]
        pub struct ResponseDto {
            pub token: String,
        }
        ResponseDto { token: utils::auth::create_jwt(&ctx.deps, Identity::User(output))? }
    }))
}
