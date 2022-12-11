use actix_web::{cookie::Cookie, http::header::HeaderMap, web, HttpRequest, HttpResponse, Result};
use async_graphql::{
    dataloader::DataLoader,
    http::{playground_source, GraphQLPlaygroundConfig},
    EmptySubscription, MergedObject, Schema,
};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};

use crate::{
    auth::resolver::AuthMutation,
    common::resolver::CommonQuery,
    config::{Cache, Database, Jwt, Mailer},
    loaders::SeaOrmLoader,
    users::resolver::{UsersMutation, UsersQuery},
};

#[derive(Clone)]
pub struct Environment(pub String);

#[derive(MergedObject, Default)]
pub struct MutationRoot(UsersMutation, AuthMutation);

#[derive(MergedObject, Default)]
pub struct QueryRoot(CommonQuery, UsersQuery);

fn get_access_token_from_headers(headers: &HeaderMap) -> Option<String> {
    let auth_header = headers.get("Authorization");

    if auth_header.is_none() {
        return None;
    }

    let auth_header = auth_header.unwrap();

    if auth_header.is_empty() {
        return None;
    }

    let auth_header = auth_header.to_str().unwrap();

    if !auth_header.starts_with("Bearer ") {
        return None;
    }

    let token_arr: Vec<&str> = auth_header.split_whitespace().collect();
    let token = token_arr.get(1);

    if token.is_none() {
        return None;
    }

    let token = token.unwrap().to_owned();

    if token.is_empty() {
        return None;
    }

    Some(token.to_string())
}

fn get_refresh_token_from_cookie(cookie: Option<Cookie>) -> Option<String> {
    if cookie.is_none() {
        return None;
    }

    let cookie = cookie.unwrap();

    if cookie.value().is_empty() {
        return None;
    }

    Some(cookie.value().to_string())
}

pub struct AuthTokens {
    pub access_token: Option<String>,
    pub refresh_token: Option<String>,
}

impl AuthTokens {
    pub fn new(request: &HttpRequest) -> Self {
        Self {
            access_token: get_access_token_from_headers(request.headers()),
            refresh_token: get_refresh_token_from_cookie(request.cookie("refresh_token")),
        }
    }
}

pub async fn gql_index_playground() -> Result<HttpResponse> {
    let source = playground_source(GraphQLPlaygroundConfig::new("/api/graphql"));
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(source))
}

pub async fn gql_index(
    schema: web::Data<Schema<QueryRoot, MutationRoot, EmptySubscription>>,
    req: HttpRequest,
    gql_req: GraphQLRequest,
) -> GraphQLResponse {
    schema
        .execute(gql_req.into_inner().data(AuthTokens::new(&req)))
        .await
        .into()
}

pub fn build_schema(
    cache: &Cache,
    db: &Database,
    jwt: &Jwt,
    mailer: &Mailer,
    environment: &str,
) -> Schema<QueryRoot, MutationRoot, EmptySubscription> {
    Schema::build(
        QueryRoot::default(),
        MutationRoot::default(),
        EmptySubscription,
    )
    .data(DataLoader::new(SeaOrmLoader::new(db), tokio::task::spawn))
    .data(cache.to_owned())
    .data(db.to_owned())
    .data(jwt.to_owned())
    .data(mailer.to_owned())
    .data(Environment(environment.to_string()))
    .enable_federation()
    .finish()
}
