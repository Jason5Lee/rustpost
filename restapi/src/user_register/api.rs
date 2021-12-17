use super::Command;
use crate::common::*;
use actix_web::{error::ErrorUnprocessableEntity, post, web::Json as BodyJson, HttpResponse, Result};
use serde::{Deserialize, Serialize};

#[post("/register")]
pub async fn api(mut ctx: utils::Context) -> Result<HttpResponse> {
    #[derive(Deserialize)]
    #[allow(non_snake_case)]
    pub struct RequestDto {
        pub userName: String,
        pub password: String,
    }

    let req = ctx.to::<BodyJson<RequestDto>>().await?.0;
    let input = Command {
        user_name: UserName::try_new(req.userName)
            .map_err(|err| ErrorUnprocessableEntity(err.body))?,
        password: Password::from_plain(req.password)
            .map_err(|err| ErrorUnprocessableEntity(err.body))?,
    };
    let output = super::Steps::from_ctx(&ctx).workflow(input).await?;
    let user_id = utils::format_id(output.0);
    Ok(HttpResponse::Created()
        .header("Location", iformat!("/user/" user_id))
        .json({
            #[derive(Serialize)]
            #[allow(non_snake_case)]
            pub struct ResponseDto {
                pub userId: String,
            }

            ResponseDto {
                userId: user_id,
            }
        }))
}
