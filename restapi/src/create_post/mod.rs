pub mod api;
pub mod deps;
pub mod error;

use crate::{common::*, define_steps};
use actix_web::Result;

pub struct Command {
    pub title: Title,
    pub content: PostContent,
}

impl<'a> Steps<'a> {
    pub async fn workflow(self, caller: UserId, input: Command) -> Result<PostId> {
        self.store_post(caller, input).await
    }
}

define_steps! {
    async fn store_post(creator: UserId, input: Command) -> Result<PostId>;
}
