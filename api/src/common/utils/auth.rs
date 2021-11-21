use std::convert::TryInto;

use crate::common::*;
use actix_web::error::ErrorUnauthorized;
use aes_gcm::aead::{generic_array::GenericArray, Aead, AeadInPlace};

use rand::RngCore;

const USER_HEADER: u8 = 1;
const ADMIN_HEADER: u8 = 2;
const NONCE_LEN: usize = 12;
const TAG_LEN: usize = 16;

pub trait AuthMethod {
    type Id;

    fn no_auth_header() -> Result<Self::Id> {
        Err(error::unauthenticated())
    }
    fn with_auth_bytes(bytes: &[u8]) -> Result<Self::Id>;
}

pub struct UserOnly;

impl AuthMethod for UserOnly {
    type Id = UserId;

    fn with_auth_bytes(bytes: &[u8]) -> Result<Self::Id> {
        if bytes[0] != USER_HEADER {
            Err(error::user_only())
        } else {
            let id = u64::from_le_bytes((&bytes[1..=8]).try_into().unwrap());
            Ok(UserId(id))
        }
    }
}

pub struct Everyone;

impl AuthMethod for Everyone {
    type Id = Identity;

    fn no_auth_header() -> Result<Self::Id> {
        Ok(Identity::Guest)
    }

    fn with_auth_bytes(bytes: &[u8]) -> Result<Self::Id> {
        let id = u64::from_le_bytes((&bytes[1..=8]).try_into().unwrap());
        match bytes[0] {
            USER_HEADER => Ok(Identity::User(UserId(id))),
            ADMIN_HEADER => Ok(Identity::Admin(AdminId(id))),
            _ => Err(error::unauthenticated()),
        }
    }
}

pub fn auth<Method: AuthMethod>(ctx: &utils::Context) -> Result<Method::Id> {
    let call_time = Time::now();
    let base64 = match api::get_auth_header(ctx) {
        None => return Method::no_auth_header(),
        Some(header) => header.as_bytes(),
    };
    let id_bytes_encrypted = base64::decode_config(base64, base64::URL_SAFE_NO_PAD)
        .map_err(|err| ErrorUnauthorized(format!("IDENTITY_INVALID_BASE64: {}", err)))?;

    if id_bytes_encrypted.len() <= NONCE_LEN {
        return Err(ErrorUnauthorized("IDENTITY_INVALID"));
    }
    let (nonce, cipher) = id_bytes_encrypted.split_at(NONCE_LEN);
    let id_bytes = ctx
        .deps
        .aead
        .decrypt(GenericArray::from_slice(nonce), cipher)
        .map_err(|err| ErrorUnauthorized(format!("IDENTITY_INVALID: {}", err)))?;

    if id_bytes.len() != 17 {
        return Err(ErrorUnauthorized("IDENTITY_INVALID"));
    }
    let expiration = Time {
        utc: u64::from_le_bytes((&id_bytes[9..17]).try_into().unwrap()),
    };
    if call_time.utc > expiration.utc {
        return Err(error::identity_expired());
    }

    Method::with_auth_bytes(&id_bytes)
}

// pub fn fresh_token(&self, ctx: &Context) -> String {
//     let mut data = vec![0; NONCE_LEN + 17 + TAG_LEN];
//     let (nonce, in_out) = data.split_at_mut(NONCE_LEN);
//     let (in_out, tag) = in_out.split_at_mut(17);
//     let (id_bytes, expire) = in_out.split_at_mut(9);

//     id_bytes.copy_from_slice(&self.bytes);
//     expire.copy_from_slice(&(super::current_timestamp() + ctx.deps.id_alive_millis).to_le_bytes());

//     let mut rng = rand::thread_rng();
//     rng.fill_bytes(nonce);
//     let nonce = GenericArray::clone_from_slice(nonce);

//     let aad_tag = ctx
//         .deps
//         .aead
//         .encrypt_in_place_detached(&nonce, b"", in_out)
//         .unwrap();

//     tag.copy_from_slice(&aad_tag);

//     base64::encode_config(data, base64::URL_SAFE_NO_PAD)
// }

// pub fn auth<Id>(&self, auth_method: fn(&[u8]) -> Result<Id>) -> Result<Id> {
//     auth_method(&self.bytes)
// }

// fn get_identity(&self) -> Result<Identity> {
//     let id = u64::from_le_bytes((&self.bytes[1..=8]).try_into().unwrap());

//     match self.bytes[0] {
//         USER_HEADER => Ok(Identity::User(UserId(id))),
//         ADMIN_HEADER => Ok(Identity::Admin(AdminId(id))),
//         _ => Err(error::unauthorized()),
//     }
// }

// pub fn get_identity(auth: Option<Result<AuthToken>>) -> Result<(Identity, Option<AuthToken>)> {
//     match auth {
//         Some(r) => {
//             let r = r?;
//             Ok((r.get_identity()?, Some(r)))
//         }
//         None => Ok((Identity::Guest, None)),
//     }
// }

pub fn to_fresh_token<Id: ToBytes>(ctx: &utils::Context, id: Id) -> String {
    let mut data = vec![0; NONCE_LEN + 17 + TAG_LEN];
    let (nonce, in_out) = data.split_at_mut(NONCE_LEN);
    let (in_out, tag) = in_out.split_at_mut(17);
    let (id_bytes, expire) = in_out.split_at_mut(9);

    id.set_bytes(id_bytes);
    expire.copy_from_slice(&(super::current_timestamp() + ctx.deps.id_alive_millis).to_le_bytes());

    let mut rng = rand::thread_rng();
    rng.fill_bytes(nonce);
    let nonce = GenericArray::clone_from_slice(nonce);

    let aad_tag = ctx
        .deps
        .aead
        .encrypt_in_place_detached(&nonce, b"", in_out)
        .unwrap();

    tag.copy_from_slice(&aad_tag);

    base64::encode_config(data, base64::URL_SAFE_NO_PAD)
}

pub trait ToBytes {
    fn set_bytes(self, bytes: &mut [u8]);
}

impl ToBytes for UserId {
    fn set_bytes(self, bytes: &mut [u8]) {
        bytes[0] = USER_HEADER;
        (&mut bytes[1..=8]).copy_from_slice(&self.0.to_le_bytes());
    }
}

impl ToBytes for AdminId {
    fn set_bytes(self, bytes: &mut [u8]) {
        bytes[0] = ADMIN_HEADER;
        (&mut bytes[1..=8]).copy_from_slice(&self.0.to_le_bytes());
    }
}
