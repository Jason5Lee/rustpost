use actix_web::error::ErrorInternalServerError;
use super::*;

pub async fn insert_user(
    deps: &crate::common::utils::Deps,
    user_name: UserName,
    password: Password,
) -> Result<UserId> {
    let id = deps.id_gen.lock().real_time_generate() as u64;
    sqlx::query(
        "INSERT INTO users (id,username,bcrypted_password,creation_time_utc) VALUES (?,?,?,?)",
    )
    .bind(id)
    .bind(user_name.as_str())
    .bind(password.to_encrypted(deps.cost))
    .bind(Time::now().utc)
    .execute(&deps.pool)
    .await
    .map(|_| UserId(id))
    .map_err(|err| match err {
        sqlx::Error::Database(db)
            if db.code().map_or(false, |code| code == "23000")
                && db.message().ends_with("for key 'users.username'") =>
        {
            error::user_name_already_exists()
        }
        _ => {
            log::error!("{}", err);
            ErrorInternalServerError("")
        }
    })
}
