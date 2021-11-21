use crate::common::*;
use crate::define_api;
use actix_web::{delete, web::Path as UrlPath, HttpResponse, Result};

define_api!(delete("/post/{id}"), Everyone);

async fn to_input(ctx: &mut utils::Context) -> Result<PostId> {
    let (id,): (String,) = ctx.to::<UrlPath<(String,)>>().await?.0;
    Ok(PostId(utils::parse_id(&id)?))
}

pub const STATUS: utils::Status = HttpResponse::NoContent;

pub fn output_to_response(_: ()) -> HttpResponse {
    STATUS().finish()
}
