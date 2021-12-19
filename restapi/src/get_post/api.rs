use std::rc::Rc;

use crate::common::utils::Context;
use actix_web::{get, web::Path as UrlPath, HttpResponse, Result};
use apply::Apply;
use serde::Serialize;

use super::*;

#[get("/post/{id}")]
pub async fn api(mut ctx: Context) -> Result<HttpResponse> {
    let UrlPath((id,)) = ctx.to::<UrlPath<(String,)>>().await?;
    let input = utils::parse_id(&id)
        .map(PostId)
        .map_err(|_| error::post_not_found())?;
    let output = super::Steps::from_ctx(&ctx).workflow(input).await?;

    HttpResponse::Ok().json({
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

        let (post, url) = match output.content {
            PostContent::Post(post) => (Some(post), None),
            PostContent::Url(url) => (None, Some(url.to_string())),
        };

        ResponseDto {
            creatorId: utils::format_id(output.creator.id.0),
            creatorName: output.creator.name.into_rc_str(),
            creationTime: output.creation.utc,
            title: output.title.into_string(),
            post,
            url,
            lastModified: output.last_modified.map(|t| t.utc),
        }
    })
    .apply(Ok)
}
