use super::error;
use crate::common::*;
use actix_web::Result;
use sqlx::Done;

pub async fn get_post_creator(deps: &utils::Deps, post: PostId) -> Result<UserId> {
    sqlx::query_as(&iformat!("SELECT " db::posts::CREATOR " FROM " db::POSTS " WHERE " db::posts::POST_ID "=?"))
        .bind(post.0)
        .fetch_optional(&deps.pool)
        .await
        .map_err(utils::handle_internal)?
        .ok_or_else(error::post_not_found)
        .map(|(creator,): (u64,)| UserId(creator))
}

pub async fn delete_post(deps: &utils::Deps, post: PostId) -> Result<()> {
    let rows_affected =
        sqlx::query(&iformat!("DELETE FROM " db::POSTS " WHERE " db::posts::POST_ID "=?"))
            .bind(post.0)
            .execute(&deps.pool)
            .await
            .map_err(utils::handle_internal)?
            .rows_affected();
    if rows_affected != 0 {
        Ok(())
    } else {
        Err(error::post_not_found())
    }
}
