use std::rc::Rc;
use crate::common::*;
use serde::Serialize;
use actix_web::{get, HttpResponse, Result, web::Path as UrlPath};
use super::error;

#[get("/user/{id}")]
pub async fn api(mut ctx: utils::Context) -> Result<HttpResponse> {
    let (id,) = ctx.to::<UrlPath<(String,)>>().await?.0;
    let input = UserId(utils::parse_id(&id).map_err(|_| error::user_not_found())?);
    let output = super::Steps::from_ctx(&ctx).workflow(input).await?;
    Ok(HttpResponse::Ok().json({
        #[derive(Serialize)]
        #[allow(non_snake_case)]
        pub struct ResponseDto {
            pub userName: Rc<str>,
            pub creationTime: u64,
        }

        ResponseDto {
            userName: output.user_name.into_rc_str(),
            creationTime: output.creation.utc,
        }
    }))
}