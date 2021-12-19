use crate::common::*;
use actix_web::{get, web::Path as UrlPath, web::Query as QueryString, HttpResponse, Result};
use actix_web::error::ErrorBadRequest;
use super::*;
use serde::{Serialize, Deserialize};

#[get("/user/{userId}/posts")]
pub async fn api(mut ctx: utils::Context) -> Result<HttpResponse> {
    let UrlPath((creator_id,)) = ctx.to::<UrlPath<(String,)>>().await?;
    #[derive(Deserialize)]
    #[allow(non_snake_case)]
    pub struct RequestDto {
        pub before: Option<u64>,
        pub after: Option<u64>,
        pub size: Option<u32>,
    }
    let req = ctx.to::<QueryString<RequestDto>>().await?.0;

    let input = Query {
        creator: utils::parse_id(&creator_id)
            .map(UserId)
            .map_err(|_| error::creator_not_found())?,
        condition: match (req.before, req.after) {
            (None, None) => Condition::No,
            (Some(utc), None) => Condition::Before(Time { utc }),
            (None, Some(utc)) => Condition::After(Time { utc }),
            _ => return Err(ErrorBadRequest("BOTH_BEFORE_AFTER")),
        },
        size: Size::try_new(req.size)?,
    };
    let output = Steps::from_ctx(&ctx).workflow(input).await?;
    #[derive(Serialize)]
    #[allow(non_snake_case)]
    pub struct PostInfoDto {
        pub id: String,
        pub title: String,
        pub creationTime: u64,
    }
    #[derive(Serialize)]
    #[allow(non_snake_case)]
    pub struct ResponseDto {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub lastPage: Option<bool>,
        pub info: Vec<PostInfoDto>,
    }

    Ok(HttpResponse::Ok().json(ResponseDto {
        lastPage: output.last_page,
        info: output.posts.into_iter().map(|info| {
            PostInfoDto {
                id: utils::format_id(info.id.0),
                title: info.title.into_string(),
                creationTime: info.creation.utc,
            }
        }).collect::<Vec<_>>()
    }))
}