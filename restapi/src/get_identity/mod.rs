use crate::{common::*, define_steps};
use actix_web::Result;

pub mod api;
pub mod deps;

pub enum IdentityInfo {
    User { id: UserId, name: UserName },
    Admin { id: AdminId },
}

impl<'a> Steps<'a> {
    pub async fn workflow(self, caller: Option<Identity>) -> Result<Option<IdentityInfo>> {
        match caller {
            Some(Identity::User(id)) => Ok(Some(IdentityInfo::User {
                id,
                name: self.get_user_name(id).await?,
            })),
            Some(Identity::Admin(id)) => Ok(Some(IdentityInfo::Admin { id })),
            None => Ok(None),
        }
    }
}

define_steps! {
    async fn get_user_name(id: UserId) -> Result<UserName>;
}
