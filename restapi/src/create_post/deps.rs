use super::*;
use crate::common::{utils::Deps, *};

pub async fn store_post(deps: &Deps, creator: UserId, info: PostInfo) -> Result<PostId> {
    let id = deps.id_gen.lock().real_time_generate() as u64;
    let db::PostContent { post_type, content } = db::PostContent::from_model(info.content);
    sqlx::query(&iformat!("INSERT INTO " db::POSTS "(" db::posts::POST_ID "," db::posts::CREATOR "," db::posts::CREATION_TIME_UTC "," db::posts::TITLE "," db::posts::POST_TYPE "," db::posts::CONTENT ") VALUES (?,?,?,?,?,?)"))
        .bind(id)
        .bind(creator.0)
        .bind(Time::now().utc)
        .bind(info.title.as_str())
        .bind(post_type)
        .bind(content)
        .execute(&deps.pool)
        .await
        .map_err(|err|
            if db::is_unique_violation_in(&err, db::posts::TITLE) {
                error::duplicate_title()
            } else if db::is_unique_violation_in(&err, db::posts::POST_ID) {
                error_common::too_many_requests()
            } else {
                utils::handle_internal(err)
            }
        )
        .map(|_| PostId(id))
}
