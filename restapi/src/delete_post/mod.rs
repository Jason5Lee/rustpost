pub mod api;
pub mod deps;
pub mod error;
#[cfg(test)]
pub mod tests;

use crate::{common::*, define_steps};
use actix_web::Result;

pub type Command = PostId;

impl<'a> Steps<'a> {
    pub async fn workflow(self, caller: Identity, input: Command) -> Result<()> {
        let auth: bool = match caller {
            Identity::Admin(_) => true,
            Identity::User(id) => id == self.get_post_creator(input).await?,
        };

        if auth {
            self.delete_post(input).await
        } else {
            Err(error::unauthorized())
        }
    }
}

define_steps! {
    async fn get_post_creator(post: PostId) -> Result<UserId>;
    async fn delete_post(id: PostId) -> Result<()>;
}
