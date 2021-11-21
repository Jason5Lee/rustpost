use actix_web::error::ErrorInternalServerError;

use crate::common::utils::handle_internal;

use super::*;

pub async fn workflow(deps: &crate::common::utils::Deps, input: Query) -> Result<PostInfoForPage> {
    let (creator, creation_time_utc, last_modified_utc, title,post_type, content): (u64, u64, Option<u64>, String, String, String) =
        sqlx::query_as("SELECT creator,creation_time_utc,last_modified_utc,title,post_type,content FROM posts WHERE id=?")
            .bind(input.0)
            .fetch_optional(&deps.pool)
            .await
            .map_err(|err| {
                log::error!("{}", err);
                ErrorInternalServerError("")
            })?
            .ok_or_else(error::post_not_found)?;
    let (creator_name,): (String,) = sqlx::query_as("SELECT username from users WHERE id=?")
        .bind(creator)
        .fetch_optional(&deps.pool)
        .await
        .map_err(handle_internal)?
        .ok_or_else(|| {
            log::error!("post {} has creator {} which not exists", input.0, creator);
            ErrorInternalServerError("")
        })?;
    let db_content = utils::db::PostContent { post_type, content };
    Ok(PostInfoForPage {
        creator: CreatorInfo {
            name: UserName::try_new(creator_name).map_err(|err| err.persisted_invalid())?,
            id: UserId(creator),
        },
        creation: Time {
            utc: creation_time_utc,
        },
        last_modified: last_modified_utc.map(|utc| Time { utc }),
        title: Title::try_new(title).map_err(|err| err.persisted_invalid())?,
        content: db_content
            .try_into_model()
            .map_err(utils::Invalid::persisted_invalid)?,
    })
}
