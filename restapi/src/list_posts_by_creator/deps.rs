use super::*;
use crate::common::*;
use actix_web::Result;

pub async fn workflow(deps: &utils::Deps, input: Query) -> Result<Output> {
    let user_not_found = sqlx::query(&iformat!("SELECT 0 FROM `" db::USERS "` WHERE `" db::users::USER_ID "`=?"))
        .bind(input.creator.0)
        .fetch_optional(&deps.pool)
        .await
        .map_err(utils::handle_internal)?
        .is_none();

    if user_not_found {
        return Err(error::creator_not_found())
    }

    let post_condition_sql = match input.condition {
        Condition::No => iformat!("WHERE `" db::posts::CREATOR "`=?"),
        Condition::Before(before) => iformat!("WHERE `" db::posts::CREATOR "`=? AND `" db::posts::CREATION_TIME_UTC "` < " before.utc),
        Condition::After(after) => iformat!("WHERE `" db::posts::CREATOR "`=? AND `" db::posts::CREATION_TIME_UTC "` > " after.utc),
    };
    let post_sql = iformat!(
        "SELECT " db::posts::POST_ID
        "," db::posts::CREATION_TIME_UTC
        "," db::posts::TITLE
        " FROM " db::POSTS " " post_condition_sql
        " ORDER BY " db::posts::CREATION_TIME_UTC " DESC LIMIT ?"
    );
    let posts_db_result: Vec<(u64, u64, String)> = sqlx::query_as(&post_sql)
        .bind(input.creator.0)
        .bind(input.size.to_u32())
        .fetch_all(&deps.pool).await.map_err(utils::handle_internal)?;

    if posts_db_result.is_empty() {
        return Ok(Output {last_page: Some(true), posts: Vec::new()})
    }

    let posts = posts_db_result
        .into_iter()
        .map(|(id, creation_time_utc, title)| {
            Ok(PostInfo {
                id: PostId(id),
                title: Title::try_new(title).map_err(|err| err.log_then_internal_error())?,
                creation: Time {
                    utc: creation_time_utc,
                },
            })
        })
        .collect::<Result<Vec<PostInfo>>>()?;

    Ok(Output {
        last_page: if posts.len() < input.size.to_u32() as usize { Some(true) } else { None },
        posts,
    })
}