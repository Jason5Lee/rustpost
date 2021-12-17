use super::*;
use crate::common::*;
use actix_web::HttpResponse;
use actix_web::{post, web::Json as BodyJson, Result};
use serde::Deserialize;
use serde::Serialize;

#[post("/admin/login")]
pub async fn api(mut ctx: utils::Context) -> Result<HttpResponse> {
    #[derive(Deserialize)]
    pub struct RequestDto {
        pub id: String,
        pub password: String,
    }
    let req_body = ctx.to::<BodyJson<RequestDto>>().await?.0;
    let input = Query {
        id: AdminId(utils::parse_id(&req_body.id).map_err(|_| error::id_or_password_incorrect())?),
        password: Password::from_plain(req_body.password)
            .map_err(|_| error::id_or_password_incorrect())?,
    };
    let output = super::Steps::from_ctx(&ctx).workflow(input).await?;
    Ok(HttpResponse::Ok().json({
        #[derive(Serialize)]
        pub struct ResponseDto {
            token: String,
        }
        ResponseDto {
            token: utils::auth::create_jwt(&ctx.deps, Identity::Admin(output))?,
        }
    }))
}
