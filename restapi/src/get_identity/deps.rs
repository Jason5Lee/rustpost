use crate::common::*;
use actix_web::Result;

pub async fn get_user_name(deps: &utils::Deps, id: UserId) -> Result<UserName> {
    let (name,): (String,) = sqlx::query_as(&iformat!("SELECT " db::users::USER_NAME " FROM " db::USERS " WHERE " db::users::USER_ID "=?"))
        .bind(id.0)
        .fetch_one(&deps.pool)
        .await
        .map_err(utils::handle_internal)?;
    UserName::try_new(name).map_err(utils::Invalid::log_then_internal_error)
}
