use super::*;
use crate::common::utils::Deps;
use actix_web::error::ErrorInternalServerError;

pub async fn get_user_id_encrypted_password(
    deps: &Deps,
    user_name: UserName,
) -> Result<(UserId, String)> {
    let (id, encrypted_password): (u64, String) =
        sqlx::query_as(&iformat!("SELECT " db::users::USER_ID "," db::users::BCRYPTED_PASSWORD " FROM " db::USERS " WHERE " db::users::USER_NAME "=?"))
            .bind(user_name.as_str())
            .fetch_one(&deps.pool)
            .await
            .map_err(|err| match err {
                sqlx::Error::RowNotFound => error::user_name_or_password_incorrect(),
                _ => {
                    log::error!("{:?}\n\t at {}:{}", err, file!(), line!());
                    ErrorInternalServerError("")
                }
            })?;
    Ok((UserId(id), encrypted_password))
}
