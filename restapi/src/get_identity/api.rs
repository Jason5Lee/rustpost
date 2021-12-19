use std::rc::Rc;

use crate::common::*;
use actix_web::{get, HttpResponse, Result};
use apply::Apply;
use serde::Serialize;

use super::*;

#[get("/identity")]
pub async fn api(ctx: utils::Context) -> Result<HttpResponse> {
    let caller = utils::auth::auth_optional(&ctx)?;
    let output = super::Steps::from_ctx(&ctx).workflow(caller).await?;
    HttpResponse::Ok()
        .json({
            #[derive(Serialize, Default)]
            pub struct ResponseDto {
                #[serde(skip_serializing_if = "Option::is_none")]
                user: Option<UserDto>,
                #[serde(skip_serializing_if = "Option::is_none")]
                admin: Option<AdminDto>,
            }

            #[derive(Serialize)]
            pub struct UserDto {
                pub name: Rc<str>,
                pub id: String,
            }

            #[derive(Serialize)]
            pub struct AdminDto {
                pub id: String,
            }

            match output {
                Some(IdentityInfo::User { id, name }) => ResponseDto {
                    user: Some(UserDto {
                        name: name.into_rc_str(),
                        id: utils::format_id(id.0),
                    }),
                    ..Default::default()
                },
                Some(IdentityInfo::Admin { id }) => ResponseDto {
                    admin: Some(AdminDto {
                        id: utils::format_id(id.0),
                    }),
                    ..Default::default()
                },
                None => Default::default(),
            }
        })
        .apply(Ok)
}
