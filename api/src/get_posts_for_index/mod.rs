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

pub struct PostInfoForPage {
    pub id: PostId,
    pub title: Title,
    pub creator: CreatorInfo,
    pub creation_date: Time,
}

pub struct Output {
    pub exhausted: bool,
    pub posts: Vec<PostInfoForPage>,
}
define_steps! {
    async fn workflow(input: Query) -> Result<Output>;
}
