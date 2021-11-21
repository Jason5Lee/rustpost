use crate::{common::*, define_steps};
use actix_web::Result;

pub mod api;
pub mod deps;

pub enum IdentityInfo {
    User { id: UserId, name: UserName },
    Admin { id: AdminId },
    Guest,
}

impl<'a> Steps<'a> {
    pub async fn workflow(self, caller: Identity, _: ()) -> Result<IdentityInfo> {
        match caller {
            Identity::User(id) => Ok(IdentityInfo::User {
                id,
                name: self.get_user_name(id).await?,
            }),
            Identity::Admin(id) => Ok(IdentityInfo::Admin { id }),
            Identity::Guest => Ok(IdentityInfo::Guest),
        }
    }
}

define_steps! {
    async fn get_user_name(id: UserId) -> Result<UserName>;
}
