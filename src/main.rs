use actix_web::{App, HttpServer};
use dotenvy::dotenv;
use std::env;

use graphql_local_oauth::{
    app::configure_app,
    config::{Cache, Database, Jwt, Mailer},
};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let cache = Cache::new();
    let db = Database::new().await;
    let jwt = Jwt::new();
    let mailer = Mailer::new();
    let port = env::var("PORT").unwrap().parse::<u16>().unwrap();
    let env_type = env::var("ENV_TYPE").unwrap();
    let env_copy = env_type.clone();

    HttpServer::new(move || {
        App::new().configure(configure_app(&cache, &db, &jwt, &mailer, &env_type))
    })
    .bind((
        match env_copy.as_str() {
            "production" => "0.0.0.0",
            _ => "127.0.0.1",
        },
        port,
    ))?
    .run()
    .await
}
