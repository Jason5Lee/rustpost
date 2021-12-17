use std::rc::Rc;

use super::*;
use crate::common::utils::Context;
use actix_web::{error::ErrorBadRequest, get, web::Query as QueryString, HttpResponse, Result};
use apply::Apply;
use serde::{Deserialize, Serialize};

#[get("/posts")]
pub async fn api(mut ctx: Context) -> Result<HttpResponse> {
    #[derive(Deserialize)]
    #[allow(non_snake_case)]
    pub struct RequestDto {
        pub before: Option<u64>,
        pub after: Option<u64>,
        pub size: Option<u32>,
    }
    let req = ctx.to::<QueryString<RequestDto>>().await?.0;
    let input = Query {
        condition: match (req.before, req.after) {
            (None, None) => Condition::No,
            (Some(utc), None) => Condition::Before(Time { utc }),
            (None, Some(utc)) => Condition::After(Time { utc }),
            _ => return Err(ErrorBadRequest("BOTH_BEFORE_AFTER")),
        },
        size: Size::try_new(req.size)?,
    };
    let output = super::Steps::from_ctx(&ctx).workflow(input).await?;

    HttpResponse::Ok().json({
        #[derive(Serialize)]
        #[allow(non_snake_case)]
        pub struct ResponseDto {
            pub lastPage: bool,
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

        ResponseDto {
            lastPage: output.last_page,
            info: output
                .posts
                .into_iter()
                .map(|output| PostInfoForPageDto {
                    id: utils::format_id(output.id.0),
                    title: output.title.into_string(),
                    creatorName: output.creator.name.into_rc_str(),
                    creatorId: utils::format_id(output.creator.id.0),
                    creationTime: output.creation.utc,
                })
                .collect::<Vec<_>>(),
        }
    })
    .apply(Ok)
}
