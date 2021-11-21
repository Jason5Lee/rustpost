use sqlx::Done;

use super::error;
use crate::common::*;
use actix_web::Result;

pub async fn checks_user_is_creator_and_content_has_the_same_post_type(
    deps: &utils::Deps,
    post: PostId,
    user: UserId,
    content: &PostContent,
) -> Result<()> {
    let (creator, post_type): (u64, String) =
        sqlx::query_as("SELECT creator,post_type FROM posts WHERE id=?")
            .bind(post.0)
            .fetch_optional(&deps.pool)
            .await
            .map_err(utils::handle_internal)?
            .ok_or_else(error::post_not_found)?;
    if creator != user.0 {
        return Err(error::not_creator());
    }
    let content_type_matched = match content {
        PostContent::Post(_) => post_type == "post",
        PostContent::Url(_) => post_type == "url",
    };

    if !content_type_matched {
        return Err(error::content_type_diff());
    }

    Ok(())
}

pub async fn update_post(deps: &utils::Deps, post: PostId, new_content: PostContent) -> Result<()> {
    let new_content = utils::db::PostContent::from_model(new_content).content;

    let done = sqlx::query("UPDATE posts SET content=? WHERE id=?")
        .bind(new_content)
        .bind(post.0)
        .execute(&deps.pool)
        .await
        .map_err(utils::handle_internal)?;

    if done.rows_affected() == 0 {
        Err(error::post_not_found())
    } else {
        Ok(())
    }
}
