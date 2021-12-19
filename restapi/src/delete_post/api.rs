use super::*;
use crate::common::*;
use actix_web::{delete, web::Path as UrlPath, HttpResponse, Result};
use apply::Apply;

#[delete("/post/{id}")]
pub async fn api(mut ctx: utils::Context) -> Result<HttpResponse> {
    let caller = utils::auth::auth(&ctx)?;
    let (id,): (String,) = ctx.to::<UrlPath<(String,)>>().await?.0;
    let input = utils::parse_id(&id)
        .map_err(|_| error::post_not_found())?
        .apply(PostId);
    super::Steps::from_ctx(&ctx).workflow(caller, input).await?;
    Ok(HttpResponse::NoContent().finish())
}
