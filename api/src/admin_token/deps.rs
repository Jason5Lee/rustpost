use super::error;
use super::*;
use crate::common::*;

pub async fn workflow(deps: &utils::Deps, input: Query) -> Result<AdminId> {
    let (encrypted_password,): (String,) =
        sqlx::query_as("SELECT bcrypted_password FROM admins WHERE id=?")
            .bind(input.id.0)
            .fetch_optional(&deps.pool)
            .await
            .map_err(utils::handle_internal)?
            .ok_or_else(error::id_or_password_incorrect)?;
    if input.password.verify_encrypted(&encrypted_password) {
        Ok(input.id)
    } else {
        Err(error::id_or_password_incorrect())
    }
}
