use super::*;

pub async fn insert_user(
    deps: &crate::common::utils::Deps,
    user_name: UserName,
    password: Password,
) -> Result<UserId> {
    let id = deps.id_gen.lock().real_time_generate() as u64;
    sqlx::query(&iformat!(
        "INSERT INTO " db::USERS " (" db::users::USER_ID "," db::users::USER_NAME "," db::users::BCRYPTED_PASSWORD "," db::users::CREATION_TIME_UTC ") VALUES (?,?,?,?)"
    ))
        .bind(id)
        .bind(user_name.as_str())
        .bind(password.to_encrypted(&deps.encryptor)?)
        .bind(Time::now().utc)
        .execute(&deps.pool)
        .await
        .map(|_| UserId(id))
        .map_err(|err|
            if db::is_unique_violation_in(&err, db::users::USER_NAME) {
                error::user_name_already_exists()
            } else if db::is_unique_violation_in(&err, db::users::USER_ID) {
                error_common::too_many_requests()
            } else {
                utils::handle_internal(err)
            }
        )
}
