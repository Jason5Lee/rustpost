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

pub struct PostInfo {
    pub id: PostId,
    pub title: Title,
    pub creator: UserName,
    pub creation: Time,
}

pub struct Output {
    pub last_page: Option<bool>,
    pub posts: Vec<PostInfo>,
}
define_steps! {
    async fn workflow(input: Query) -> Result<Output>;
}
