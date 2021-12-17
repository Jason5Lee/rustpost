use super::error;
use super::*;
use crate::common::*;

pub async fn workflow(deps: &utils::Deps, input: Query) -> Result<AdminId> {
    let (encrypted_password,): (String,) =
        sqlx::query_as(&iformat!("SELECT " db::admin::BCRYPTED_PASSWORD " FROM " db::ADMIN " WHERE " db::admin::ADMIN_ID "=?"))
            .bind(input.id.0)
            .fetch_optional(&deps.pool)
            .await
            .map_err(utils::handle_internal)?
            .ok_or_else(error::id_or_password_incorrect)?;
    if input.password.verify(&encrypted_password)? {
        Ok(input.id)
    } else {
        Err(error::id_or_password_incorrect())
    }
}
