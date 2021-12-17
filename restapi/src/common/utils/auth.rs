use crate::common::*;
use actix_web::error::ErrorUnauthorized;

use apply::Apply;
use jsonwebtoken::{DecodingKey, EncodingKey};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[allow(non_snake_case)]
pub struct Claim {
    pub exp: u64,
    pub userId: Option<String>,
    pub adminId: Option<String>,
}

fn invalid_auth() -> actix_web::Error {
    ErrorUnauthorized("INVALID_AUTH")
}

fn get_exp(deps: &utils::Deps) -> u64 {
    (utils::current_timestamp() + deps.id_alive_millis) / 1000 // exp is seconds
}
pub fn create_jwt(deps: &utils::Deps, identity: Identity) -> Result<String> {
    let claim = match identity {
        Identity::User(user_id) => Claim {
            exp: get_exp(deps),
            userId: Some(utils::format_id(user_id.0)),
            adminId: None,
        },
        Identity::Admin(admin_id) => Claim {
            exp: get_exp(deps),
            userId: None,
            adminId: Some(utils::format_id(admin_id.0)),
        }
    };
    jsonwebtoken::encode(&jsonwebtoken::Header::default(), &claim, &EncodingKey::from_secret(&deps.secret))
        .map_err(|_| invalid_auth())
}
fn decode_jwt(deps: &utils::Deps, token: &str) -> Result<Claim> {
    jsonwebtoken::decode::<Claim>(token, &DecodingKey::from_secret(&deps.secret), &jsonwebtoken::Validation::default())
        .map(|data| data.claims)
        .map_err(|err| match err.into_kind() {
            jsonwebtoken::errors::ErrorKind::ExpiredSignature => error_common::auth_expired(),
            _ => invalid_auth(),
        })
}

fn decode_jwt_from_auth_header(deps: &utils::Deps, header: &actix_web::http::HeaderValue) -> Result<Claim> {
    header.as_bytes()
        .strip_prefix(b"Bearer ")
        .ok_or_else(invalid_auth)
        .and_then(|token| {
            let token =
                std::str::from_utf8(token).map_err(|_| invalid_auth())?;
            decode_jwt(deps, token)
        })
}

fn get_claim(ctx: &utils::Context) -> Result<Claim> {
    let token_header = ctx
        .request
        .headers()
        .get("Authorization")
        .ok_or_else(error_common::unauthenticated)?;

    decode_jwt_from_auth_header(&ctx.deps, token_header)
}

fn get_claim_optional(ctx: &utils::Context) -> Result<Option<Claim>> {
    match ctx.request.headers().get("Authorization") {
        None => Ok(None),
        Some(token_header) => {
            Ok(Some(decode_jwt_from_auth_header(&ctx.deps, token_header)?))
        }
    }
}

pub fn auth_user_only(ctx: &utils::Context) -> Result<UserId> {
    let claim = get_claim(ctx)?;
    if let Some(user_id) = claim.userId {
        utils::parse_id(&user_id)
            .map(UserId)
            .map_err(|err| ErrorUnauthorized(err.body))
    } else {
        Err(error::user_only())
    }
}

pub fn auth(ctx: &utils::Context) -> Result<Identity> {
    let claim = get_claim(ctx)?;
    if let Some(user_id) = claim.userId {
        utils::parse_id(&user_id)
            .map(|id| Identity::User(UserId(id)))
            .map_err(|err| ErrorUnauthorized(err.body))
    } else if let Some(admin_id) = claim.adminId {
        utils::parse_id(&admin_id)
            .map(|id| Identity::Admin(AdminId(id)))
            .map_err(|err| ErrorUnauthorized(err.body))
    } else {
        Err(error::unauthenticated())
    }
}

pub fn auth_optional(ctx: &utils::Context) -> Result<Option<Identity>> {
    match get_claim_optional(ctx)? {
        None => Ok(None),
        Some(claim) => {
            if let Some(user_id) = claim.userId {
                utils::parse_id(&user_id)
                    .map(|id| Identity::User(UserId(id)).apply(Some))
                    .map_err(|err| ErrorUnauthorized(err.body))
            } else if let Some(admin_id) = claim.adminId {
                utils::parse_id(&admin_id)
                    .map(|id| Identity::Admin(AdminId(id)).apply(Some))
                    .map_err(|err| ErrorUnauthorized(err.body))
            } else {
                Err(error::unauthenticated())
            }
        }
    }
}
