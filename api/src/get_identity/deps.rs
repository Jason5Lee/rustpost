use crate::common::*;
use actix_web::Result;

pub async fn get_user_name(deps: &utils::Deps, id: UserId) -> Result<UserName> {
    sqlx::query_as("SELECT username FROM users WHERE id=?")
        .bind(id.0)
        .fetch_one(&deps.pool)
        .await
        .map_err(utils::handle_internal)
        .and_then(|(name,): (String,)| {
            UserName::try_new(name).map_err(utils::Invalid::persisted_invalid)
        })
}
