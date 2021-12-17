use actix_web::http::HeaderValue;

use crate::common::utils::Context;

const AUTHORIZATION: &str = "Authorization";
pub fn get_auth_header(ctx: &Context) -> Option<&HeaderValue> {
    ctx.request.headers().get(AUTHORIZATION)
}
