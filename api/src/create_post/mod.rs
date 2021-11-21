pub mod api;
pub mod deps;
pub mod error;

use crate::{common::*, define_steps};
use actix_web::Result;

pub struct Command {
    pub info: PostInfo,
}

impl<'a> Steps<'a> {
    pub async fn workflow(self, caller: UserId, input: Command) -> Result<PostId> {
        self.store_post(caller, input.info).await
    }
}

define_steps! {
    async fn store_post(creator: UserId, info: PostInfo) -> Result<PostId>;
}
