use super::*;
use crate::common::utils::{handle_internal, Deps};
use actix_web::error::ErrorInternalServerError;
use futures_util::{StreamExt, TryStreamExt};
use std::{collections::HashMap, io::Write};

pub async fn workflow(deps: &Deps, Query { condition, size }: Query) -> Result<Output> {
    // For convenient I don't use binding for time range.
    // I hope it won't cause injection
    let post_condition_sql = match condition {
        Condition::No => String::new(),
        Condition::Before(before) => iformat!("WHERE " db::posts::CREATION_TIME_UTC " < " before.utc),
        Condition::After(after) => iformat!("WHERE " db::posts::CREATION_TIME_UTC " > " after.utc),
    };
    let post_sql = iformat!(
        "SELECT " db::posts::POST_ID
        "," db::posts::CREATOR
        "," db::posts::CREATION_TIME_UTC
        "," db::posts::TITLE
        " FROM " db::POSTS " " post_condition_sql
        " ORDER BY " db::posts::CREATION_TIME_UTC " DESC LIMIT ?"
    );
    let posts_db_result: Vec<(u64, u64, u64, String)> = sqlx::query_as(&post_sql).bind(size.to_u32())
        .fetch_all(&deps.pool).await.map_err(utils::handle_internal)?;

    if posts_db_result.is_empty() {
        return Ok(Output {last_page: true, posts: Vec::new()})
    }
    // Because sqlx doesn't support binding a slice for MySQL.
    // I have to manually make the query.
    // I hope it won't cause injection.
    let mut users_query: Vec<u8> = iformat!("SELECT " db::users::USER_ID "," db::users::USER_NAME " FROM " db::USERS " WHERE " db::users::USER_ID " IN (").into();
    for (_, creator, _, _) in posts_db_result.iter() {
        write!(&mut users_query, "{},", *creator).unwrap();
    }
    // replacing the trailing comma cause MySQL doesn't support it.
    *users_query.last_mut().unwrap() = b')';

    let user_id_to_name: HashMap<u64, UserName> =
        sqlx::query_as(&String::from_utf8(users_query).unwrap())
            .fetch(&deps.pool)
            .map(|r| -> Result<(u64, UserName)> {
                let (id, username): (u64, String) = r.map_err(utils::handle_internal)?;
                Ok((
                    id,
                    UserName::try_new(username).map_err(utils::Invalid::log_then_internal_error)?,
                ))
            })
            .try_collect()
            .await
            .map_err(handle_internal)?;

    let posts = posts_db_result
        .into_iter()
        .map(|(id, creator, creation_time_utc, title)| {
            Ok(PostInfoForIndex {
                id: PostId(id),
                title: Title::try_new(title).map_err(|err| err.log_then_internal_error())?,
                creator: CreatorInfo {
                    name: user_id_to_name
                        .get(&creator)
                        .ok_or_else(|| {
                            log::error!("creator of post {}, {}, not found in users", id, creator);
                            ErrorInternalServerError("")
                        })?
                        .clone(),
                    id: UserId(creator),
                },
                creation: Time {
                    utc: creation_time_utc,
                },
            })
        })
        .collect::<Result<Vec<PostInfoForIndex>>>()?;

    Ok(Output {
        last_page: posts.len() < size.to_u32() as usize,
        posts,
    })
}
