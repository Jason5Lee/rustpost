use actix_web::error::ErrorInternalServerError;

use super::*;
use crate::common::utils::Deps;

pub async fn store_post(deps: &Deps, creator: UserId, info: PostInfo) -> Result<PostId> {
    let id = deps.id_gen.lock().real_time_generate() as u64;
    let utils::db::PostContent { post_type, content } =
        utils::db::PostContent::from_model(info.content);
    sqlx::query("INSERT INTO posts (id,creator,creation_time_utc,title,post_type,content) VALUES (?, ?, ?, ?, ?)")
        .bind(id)
        .bind(creator.0)
        .bind(Time::now().utc)
        .bind(info.title.as_str())
        .bind(post_type)
        .bind(content)
        .execute(&deps.pool)
        .await
        .map_err(|err| {
            match err {
                sqlx::Error::Database(db_error) if db_error.code().map_or(false, |code| code == "23000") => error::duplicate_title(),
                _ => {
                    log::error!("{:?}\n\t at {}:{}", err, file!(), line!());
                    ErrorInternalServerError("")
                }
            }
        })
        .map(|_| PostId(id))
}
