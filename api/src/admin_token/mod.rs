pub mod api;
pub mod deps;
pub mod error;

use crate::{common::*, define_steps};
use actix_web::Result;

pub struct Query {
    id: AdminId,
    password: Password,
}

define_steps! {
    async fn workflow(input: Query) -> Result<AdminId>;
}
