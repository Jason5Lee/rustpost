use crate::common::utils::handle_internal;
use super::*;

pub async fn workflow(deps: &crate::common::utils::Deps, input: Query) -> Result<UserInfoForPage> {
    let (user_name, creation_time_utc): (String, u64) = sqlx::query_as(&iformat!("SELECT " db::users::USER_NAME "," db::users::CREATION_TIME_UTC " FROM " db::USERS " WHERE " db::users::USER_ID "=?"))
        .bind(input.0)
        .fetch_optional(&deps.pool)
        .await
        .map_err(handle_internal)?
        .ok_or_else(error::user_not_found)?;
    Ok(UserInfoForPage {
        user_name: UserName::try_new(user_name).map_err(utils::Invalid::log_then_internal_error)?,
        creation: Time { utc: creation_time_utc }
    })
}
