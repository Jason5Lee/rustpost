use super::*;
use crate::common::utils::{handle_internal, Deps};
use actix_web::error::ErrorInternalServerError;
use futures_util::{StreamExt, TryStreamExt};
use std::{collections::HashMap, io::Write};

pub async fn workflow(deps: &Deps, Query { condition, size }: Query) -> Result<Output> {
    let posts_db_result: Vec<(u64, u64, u64, String)> = match condition {
        Condition::No => sqlx::query_as("SELECT id,creator,creation_time_utc,title FROM posts ORDER BY creation_time_utc DESC LIMIT ?").bind(size.to_u32()),
        Condition::Before(before) => sqlx::query_as("SELECT id,creator,creation_time_utc,title FROM posts WHERE creation_time_utc < ? ORDER BY creation_time_utc DESC LIMIT ?").bind(before.utc).bind(size.to_u32()),
        Condition::After(after) => sqlx::query_as("SELECT id,creator,creation_time_utc,title FROM posts WHERE creation_time_utc > ? ORDER BY creation_time_utc DESC LIMIT ?").bind(after.utc).bind(size.to_u32()),
    }.fetch_all(&deps.pool).await.map_err(|err| {
        log::error!("{}\n\tat {}:{}", err, file!(), line!());
        ErrorInternalServerError("")
    })?;

    // Because sqlx doesn't support binding a slice for MySQL.
    // I have to manually make the query.
    // I hope it doesn't have injection.
    let mut users_query: Vec<u8> = b"SELECT id,username FROM users WHERE id IN (".to_vec();
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
                    UserName::try_new(username).map_err(utils::Invalid::persisted_invalid)?,
                ))
            })
            .try_collect()
            .await
            .map_err(handle_internal)?;

    let posts = posts_db_result
        .into_iter()
        .map(|(id, creator, creation_time_utc, title)| {
            Ok(PostInfoForPage {
                id: PostId(id),
                title: Title::try_new(title).map_err(|err| err.persisted_invalid())?,
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
                creation_date: Time {
                    utc: creation_time_utc,
                },
            })
        })
        .collect::<Result<Vec<PostInfoForPage>>>()?;

    Ok(Output {
        exhausted: posts.len() < size.to_u32() as usize,
        posts,
    })
}
