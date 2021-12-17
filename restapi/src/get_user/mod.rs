pub mod api;
pub mod deps;
pub mod error;

use crate::{common::*, define_steps};
use actix_web::Result;

pub type Query = UserId;

pub struct UserInfoForPage {
    pub user_name: UserName,
    pub creation: Time,
}

define_steps! {
    async fn workflow(id: Query) -> Result<UserInfoForPage>;
}
