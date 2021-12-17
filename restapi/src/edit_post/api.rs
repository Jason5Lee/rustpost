use super::*;
use crate::common::*;
use actix_web::error::ErrorUnprocessableEntity;
use actix_web::{
    error::ErrorBadRequest, post, web::Json as BodyJson, web::Path as UrlPath, HttpResponse, Result,
};
use serde::Deserialize;

// define_api!(post("/post"), UserOnly);
#[post("/post/{id}")]
pub async fn api(mut ctx: utils::Context) -> Result<HttpResponse> {
    let caller = utils::auth::auth_user_only(&mut ctx)?;
    #[derive(Deserialize)]
    #[allow(non_snake_case)]
    pub struct RequestDto {
        pub post: Option<String>,
        pub url: Option<String>,
    }

    let req = ctx.to::<BodyJson<RequestDto>>().await?.0;
    let input = Command {
        id: PostId(
            utils::parse_id(&ctx.to::<UrlPath<(String,)>>().await?.0 .0)
                .map_err(|err| ErrorUnprocessableEntity(err.body))?,
        ),
        new_content: match (req.post, req.url) {
            (Some(post), None) => PostContent::Post(post),
            (None, Some(url)) => {
                PostContent::parse_url(url).map_err(|err| ErrorUnprocessableEntity(err.body))?
            }
            _ => return Err(ErrorBadRequest("ONLY_EXACT_ONE_OF_POST_URL")),
        },
    };
    super::Steps::from_ctx(&ctx).workflow(caller, input).await?;
    Ok(HttpResponse::NoContent().finish())
}
