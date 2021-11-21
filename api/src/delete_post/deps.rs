use super::error;
use crate::common::*;
use actix_web::Result;
use sqlx::Done;

pub async fn is_admin(deps: &utils::Deps, admin: AdminId) -> Result<bool> {
    sqlx::query_as("SELECT EXISTS (SELECT * FROM admins WHERE id=?)")
        .bind(admin.0)
        .fetch_one(&deps.pool)
        .await
        .map_err(utils::handle_internal)
        .map(|(db_exists,): (i64,)| db_exists != 0)
}

pub async fn get_post_creator(deps: &utils::Deps, post: PostId) -> Result<UserId> {
    sqlx::query_as("SELECT creator FROM posts WHERE id=?")
        .bind(post.0)
        .fetch_optional(&deps.pool)
        .await
        .map_err(utils::handle_internal)?
        .ok_or_else(error::post_not_found)
        .map(|(creator,): (u64,)| UserId(creator))
}

pub async fn delete_post(deps: &utils::Deps, post: PostId) -> Result<()> {
    let done = sqlx::query("DELETE FROM posts WHERE id=?")
        .bind(post.0)
        .execute(&deps.pool)
        .await
        .map_err(utils::handle_internal)?;
    if done.rows_affected() != 0 {
        Ok(())
    } else {
        Err(error::post_not_found())
    }
}
