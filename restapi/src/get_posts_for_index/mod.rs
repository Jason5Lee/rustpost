pub mod api;
pub mod deps;

use crate::{common::*, define_steps};
use actix_web::Result;

pub enum Condition {
    No,
    Before(Time),
    After(Time),
}
pub struct Query {
    condition: Condition,
    size: Size,
}

pub struct CreatorInfo {
    pub name: UserName,
    pub id: UserId,
}

pub struct PostInfoForIndex {
    pub id: PostId,
    pub title: Title,
    pub creator: CreatorInfo,
    pub creation: Time,
}

pub struct Output {
    pub last_page: bool,
    pub posts: Vec<PostInfoForIndex>,
}
define_steps! {
    async fn workflow(input: Query) -> Result<Output>;
}
