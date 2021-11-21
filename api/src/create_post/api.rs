use super::*;
use crate::common::*;
use crate::define_api;

use actix_web::error::ErrorBadRequest;
use actix_web::put;
use actix_web::web::Json as JsonBody;
use actix_web::{HttpResponse, Result};
use serde::{Deserialize, Serialize};

define_api! {put("/post"), UserOnly}

#[derive(Deserialize)]
#[allow(non_snake_case)]
pub struct RequestDto {
    pub title: String,
    pub post: Option<String>,
    pub url: Option<String>,
}

async fn to_input(ctx: &mut utils::Context) -> Result<Command> {
    let req_body = ctx.to::<JsonBody<RequestDto>>().await?.0;
    Ok(Command {
        info: PostInfo {
            title: Title::try_new(req_body.title).map_err(utils::Invalid::unprocessable_entity)?,
            content: match (req_body.post, req_body.url) {
                (Some(post), None) => PostContent::Post(post),
                (None, Some(url)) => {
                    PostContent::parse_url(url).map_err(utils::Invalid::unprocessable_entity)?
                }
                _ => return Err(ErrorBadRequest("ONLY_EXACT_ONE_OF_POST_URL")),
            },
        },
    })
}

const STATUS: utils::Status = HttpResponse::Created;

#[derive(Serialize)]
#[allow(non_snake_case)]
pub struct ResponseDto {
    pub postId: String,
}

fn output_to_response(post_id: PostId) -> HttpResponse {
    let post_id: String = utils::format_id(post_id.0);
    STATUS()
        .header("Location", format!("/post/{}", post_id))
        .json(ResponseDto { postId: post_id })
}
