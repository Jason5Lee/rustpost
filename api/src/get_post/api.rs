use std::rc::Rc;

use crate::{
    common::utils::{Context, Status},
    define_api,
};
use actix_web::{
    get,
    web::{self},
    HttpResponse, Result,
};
use serde::{Serialize};

use super::*;

define_api! {get("/post/{id}")}

async fn to_input(ctx: &mut Context) -> Result<Query> {
    let web::Path((id,)) = ctx.to::<web::Path<(String,)>>().await?;
    Ok(PostId(utils::parse_id(&id)?))
}

pub const STATUS: Status = HttpResponse::Ok;

#[derive(Serialize)]
#[allow(non_snake_case)]
pub struct ResponseDto {
    pub creatorId: String,
    pub creatorName: Rc<str>,
    pub creationTime: u64,
    pub title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub post: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lastModified: Option<u64>,
}

fn output_to_response(output: PostInfoForPage) -> HttpResponse {
    let (post, url) = match output.content {
        PostContent::Post(post) => (Some(post), None),
        PostContent::Url(url) => (None, Some(url.to_string())),
    };
    STATUS().json(ResponseDto {
        creatorId: utils::format_id(output.creator.id.0),
        creatorName: output.creator.name.into_rc_str(),
        creationTime: output.creation.utc,
        title: output.title.into_string(),
        post,
        url,
        lastModified: output.last_modified.map(|t| t.utc),
    })
}
