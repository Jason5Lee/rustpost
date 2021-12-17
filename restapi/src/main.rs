use actix_web::{middleware::Logger, App, HttpServer};
use serde::Deserialize;
use sqlx::mysql::MySqlPoolOptions;
use rustpost_api::common::utils::Encryptor;

#[actix_web::main]
async fn main() {
    dotenv::dotenv().ok();

    #[derive(Deserialize)]
    struct Config {
        machine_id: i32,
        node_id: i32,
        mysql_max_connections: Option<u32>,
        mysql_uri: String,
        secret_key: String,
        id_alive: String,
        cost: Option<u32>,
    }

    let config: Config = envy::from_env().expect("unable to read config");

    let id_gen = parking_lot::Mutex::new(snowflake::SnowflakeIdGenerator::new(
        config.machine_id,
        config.node_id,
    ));
    let mut pool = MySqlPoolOptions::new();

    if let Some(mc) = config.mysql_max_connections {
        pool = pool.max_connections(mc)
    }

    let pool = pool
        .connect(&config.mysql_uri)
        .await
        .expect("unable to connect to the MySQL");

    let secret = base64::decode(&config.secret_key).expect("invalid secret key");
    let id_alive_millis = config
        .id_alive
        .parse::<humantime::Duration>()
        .expect("invalid id_alive")
        .as_millis() as u64;

    let cost = config.cost.unwrap_or(bcrypt::DEFAULT_COST);

    let deps = actix_web::web::Data::new(rustpost_api::common::utils::Deps {
        id_gen,
        pool,
        id_alive_millis,
        encryptor: Encryptor { cost },
        secret,
    });

    env_logger::init();
    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(deps.clone())
            .service(rustpost_api::create_post::api::api)
            .service(rustpost_api::user_login::api::api)
            .service(rustpost_api::get_posts_for_index::api::api)
            .service(rustpost_api::user_register::api::api)
            .service(rustpost_api::get_post::api::api)
            .service(rustpost_api::delete_post::api::api)
            .service(rustpost_api::edit_post::api::api)
            .service(rustpost_api::admin_login::api::api)
            .service(rustpost_api::get_identity::api::api)
            .service(rustpost_api::get_user::api::api)
    })
    .bind("localhost:8432")
    .expect("unable to bind")
    .run()
    .await
    .expect("unable to run http server")
}
