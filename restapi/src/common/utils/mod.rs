use std::{
    borrow::Cow,
    convert::{Infallible, TryInto},
    fmt::Debug,
};

use actix_web::{
    dev::Payload,
    error::ErrorInternalServerError,
    Error, FromRequest, HttpRequest, Result,
};
use futures_util::future::{ready, Ready};

use crate::common::utils;

pub mod auth;
pub mod macros;

pub struct Context {
    pub request: HttpRequest,
    pub payload: Payload,
    pub deps: DataDeps,
}

impl Context {
    pub fn to<T: FromRequest>(&mut self) -> T::Future {
        T::from_request(&self.request, &mut self.payload)
    }

    pub fn discard_body(&mut self) {
        drop(self.payload.take())
    }

    pub fn extract<T: FromRequest>(&self) -> T::Future {
        T::extract(&self.request)
    }
}
impl<'a> FromRequest for Context {
    type Error = Infallible;
    type Future = Ready<Result<Context, Infallible>>;
    type Config = ();

    fn from_request(req: &HttpRequest, payload: &mut actix_web::dev::Payload) -> Self::Future {
        ready(Ok(Context {
            request: req.clone(),
            payload: payload.take(),
            deps: req
                .app_data::<DataDeps>()
                .expect("dependency not found")
                .clone(),
        }))
    }
}

pub struct Encryptor {
    pub cost: u32,
}
impl Encryptor {
    pub fn encrypt(&self, s: &str) -> Result<String> {
        bcrypt::hash(s, self.cost).map_err(utils::handle_internal)
    }

    pub fn verify(plain: &str, encrypted: &str) -> Result<bool> {
        bcrypt::verify(plain, encrypted).map_err(utils::handle_internal)
    }
}

pub struct Deps {
    pub pool: sqlx::MySqlPool,
    pub id_alive_millis: u64,
    pub encryptor: utils::Encryptor,
    pub id_gen: parking_lot::Mutex<snowflake::SnowflakeIdGenerator>,
    pub secret: Vec<u8>,
}

type DataDeps = actix_web::web::Data<Deps>;

pub struct Invalid<V> {
    value: V,
    pub body: Cow<'static, str>,
}

impl<V> Invalid<V> {
    pub fn new<B: Into<Cow<'static, str>>>(value: V, body: B) -> Invalid<V> {
        Invalid {
            value,
            body: body.into(),
        }
    }
}

impl<V: Debug> Invalid<V> {
    #[track_caller]
    pub fn log_then_internal_error(self) -> Error {
        log::error!(
            "persisted invalid found, value `{:?}`, reason `{}`: ",
            self.value,
            self.body
        );
        ErrorInternalServerError("")
    }
}
#[track_caller]
pub fn handle_internal<E: Debug>(err: E) -> Error {
    log::error!("{:?}", err);
    ErrorInternalServerError("")
}

pub fn format_id(id: u64) -> String {
    base64::encode_config(&id.to_le_bytes(), base64::URL_SAFE_NO_PAD)
}
pub fn parse_id(from: &str) -> Result<u64, Invalid<&str>> {
    let bytes = base64::decode_config(from, base64::URL_SAFE_NO_PAD)
        .map_err(|err| Invalid::new(from, format!("INVALID_ID: {}", err)))?;
    let bytes = (&bytes as &[u8])
        .try_into()
        .map_err(|_| Invalid::new(from, "INVALID_ID: wrong length"))?;
    Ok(u64::from_le_bytes(bytes))
}

pub fn current_timestamp() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_millis() as u64
}
