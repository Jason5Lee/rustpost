use super::*;
use crate::common::*;
use crate::define_api;
use actix_web::{error::ErrorBadRequest, post, web::Json as JsonBody, HttpResponse, Result};
use serde::Deserialize;

define_api!(post("/post"), UserOnly);

#[derive(Deserialize)]
#[allow(non_snake_case)]
pub struct RequestDto {
    pub id: u64,
    pub post: Option<String>,
    pub url: Option<String>,
}

async fn to_input(ctx: &mut utils::Context) -> Result<Command> {
    let req_body = ctx.to::<JsonBody<RequestDto>>().await?.0;

    Ok(Command {
        id: PostId(req_body.id),
        new_content: match (req_body.post, req_body.url) {
            (Some(post), None) => PostContent::Post(post),
            (None, Some(url)) => {
                PostContent::parse_url(url).map_err(utils::Invalid::unprocessable_entity)?
            }
            _ => return Err(ErrorBadRequest("ONLY_EXACT_ONE_OF_POST_URL")),
        },
    })
}

pub const STATUS: utils::Status = HttpResponse::NoContent;

fn output_to_response(_: ()) -> HttpResponse {
    STATUS().finish()
}
