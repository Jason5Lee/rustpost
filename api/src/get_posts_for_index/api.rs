use std::rc::Rc;

use super::*;
use crate::{
    common::utils::{Context, Status},
    define_api,
};
use actix_web::{error::ErrorBadRequest, get, web::Query as UrlQueryString, HttpResponse, Result};
use serde::{Deserialize, Serialize};

define_api! {get("/post")}

#[derive(Deserialize)]
#[allow(non_snake_case)]
pub struct RequestDto {
    pub before: Option<u64>,
    pub after: Option<u64>,
    pub size: Option<u32>,
}

async fn to_input(ctx: &mut Context) -> Result<Query> {
    let req_body = ctx.to::<UrlQueryString<RequestDto>>().await?.0;

    Ok(Query {
        condition: match (req_body.before, req_body.after) {
            (None, None) => Condition::No,
            (Some(utc), None) => Condition::Before(Time { utc }),
            (None, Some(utc)) => Condition::After(Time { utc }),
            _ => return Err(ErrorBadRequest("BOTH_BEFORE_AFTER")),
        },
        size: Size::try_new(req_body.size)?,
    })
}

const STATUS: Status = HttpResponse::Ok;

#[derive(Serialize)]
#[allow(non_snake_case)]
pub struct ResponseDto {
    pub exhausted: bool,
    pub info: Vec<PostInfoForPageDto>,
}

#[derive(Serialize)]
#[allow(non_snake_case)]
pub struct PostInfoForPageDto {
    pub id: String,
    pub title: String,
    pub creatorName: Rc<str>,
    pub creatorId: String,
    pub creationTime: u64,
}

fn output_to_response(Output { exhausted, posts }: Output) -> HttpResponse {
    STATUS().json(ResponseDto {
        exhausted,
        info: posts
            .into_iter()
            .map(|output| PostInfoForPageDto {
                id: utils::format_id(output.id.0),
                title: output.title.into_string(),
                creatorName: output.creator.name.into_rc_str(),
                creatorId: utils::format_id(output.creator.id.0),
                creationTime: output.creation_date.utc,
            })
            .collect::<Vec<_>>(),
    })
}
