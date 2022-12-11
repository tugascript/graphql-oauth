use actix_web::{guard, web};

use crate::{
    config::{Cache, Database, Jwt, Mailer},
    gql_set_up::{build_schema, gql_index, gql_index_playground},
};

pub fn configure_app(
    cache: &Cache,
    db: &Database,
    jwt: &Jwt,
    mailer: &Mailer,
    environment: &str,
) -> impl Fn(&mut web::ServiceConfig) {
    let schema = build_schema(cache, db, jwt, mailer, environment);
    move |cfg: &mut web::ServiceConfig| {
        cfg.app_data(web::Data::new(schema.clone()))
            .service(
                web::resource("/api/graphql")
                    .guard(guard::Post())
                    .to(gql_index),
            )
            .service(
                web::resource("/api/graphql")
                    .guard(guard::Get())
                    .to(gql_index_playground),
            );
    }
}
