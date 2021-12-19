pub mod deps;
pub mod error;
pub mod api;

use crate::common::*;
use crate::define_steps;
use actix_web::Result;

pub enum Condition {
    No,
    Before(Time),
    After(Time),
}

pub struct Query {
    pub creator: UserId,
    pub condition: Condition,
    pub size: Size,
}

pub struct PostInfo {
    pub id: PostId,
    pub title: Title,
    pub creation: Time,
}

pub struct Output {
    pub last_page: Option<bool>,
    pub posts: Vec<PostInfo>,
}

define_steps!{
    async fn workflow(input: Query) -> Result<Output>;
}