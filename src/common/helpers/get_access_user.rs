use async_graphql::{Context, Error, Result};

use crate::{
    auth::helpers::{decode_access_token, AuthUser},
    config::Jwt,
    gql_set_up::AuthTokens,
};

pub fn get_access_user(ctx: &Context<'_>) -> Result<AuthUser> {
    let tokens = ctx.data::<AuthTokens>()?;
    let access_token = tokens
        .access_token
        .as_ref()
        .ok_or(Error::new("Unauthorized"))?;
    let jwt = ctx.data::<Jwt>()?;

    match decode_access_token(access_token, &jwt.access.public_key) {
        Ok(user) => Ok(user),
        Err(_) => Err(Error::new("Unauthorized")),
    }
}
