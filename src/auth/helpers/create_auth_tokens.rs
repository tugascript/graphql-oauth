use crate::{config::Jwt, gql_set_up::Environment};

use super::{create_token, TokenType};
use actix_web::{cookie::time::Duration, cookie::Cookie, http::header::SET_COOKIE};
use async_graphql::{Context, Result};
use entities::user::Model;

pub fn create_auth_tokens(ctx: &Context<'_>, jwt: &Jwt, user: &Model) -> Result<String> {
    let refresh_token = create_token(
        TokenType::Refresh,
        user,
        &jwt.refresh.secret,
        jwt.refresh.exp,
        &jwt.api_id,
    )?;

    ctx.insert_http_header(
        SET_COOKIE,
        Cookie::build(jwt.refresh_cookie.to_owned(), refresh_token)
            .path("/api/graphql")
            .max_age(Duration::seconds(jwt.refresh.exp))
            .http_only(true)
            .secure(ctx.data::<Environment>()?.0 == "production")
            .finish()
            .to_string(),
    );
    create_token(
        TokenType::Access,
        user,
        &jwt.access.private_key,
        jwt.access.exp,
        &jwt.api_id,
    )
}
