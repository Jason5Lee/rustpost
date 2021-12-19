use super::*;
use crate::common::*;

use actix_web::error::{ErrorBadRequest, ErrorUnprocessableEntity};
use actix_web::put;
use actix_web::web::Json as BodyJson;
use actix_web::{HttpResponse, Result};
use serde::{Deserialize, Serialize};

#[put("/post")]
pub async fn api(mut ctx: utils::Context) -> Result<HttpResponse> {
    let caller = utils::auth::auth_user_only(&ctx)?;

    #[derive(Deserialize)]
    #[allow(non_snake_case)]
    pub struct RequestDto {
        pub title: String,
        pub post: Option<String>,
        pub url: Option<String>,
    }

    let req = ctx.to::<BodyJson<RequestDto>>().await?.0;
    let input = Command {
        title: Title::try_new(req.title).map_err(|err| ErrorUnprocessableEntity(err.body))?,
        content: match (req.post, req.url) {
            (Some(post), None) => PostContent::Post(post),
            (None, Some(url)) => {
                PostContent::parse_url(url).map_err(|err| ErrorUnprocessableEntity(err.body))?
            }
            _ => return Err(ErrorBadRequest("ONLY_EXACT_ONE_OF_POST_URL")),
        },
    };
    let output = super::Steps::from_ctx(&ctx).workflow(caller, input).await?;
    let post_id = utils::format_id(output.0);
    Ok(HttpResponse::Created()
        .header("Location", format!("/post/{}", post_id))
        .json({
            #[derive(Serialize)]
            #[allow(non_snake_case)]
            pub struct ResponseDto {
                pub postId: String,
            }
            ResponseDto { postId: post_id }
        }))
}
