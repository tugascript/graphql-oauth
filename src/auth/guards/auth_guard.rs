use async_graphql::{async_trait, Error, Guard, Result};

use crate::gql_set_up::AuthTokens;

pub struct AuthGuard;

#[async_trait::async_trait]
impl Guard for AuthGuard {
    async fn check(&self, ctx: &async_graphql::Context<'_>) -> Result<()> {
        let tokens = ctx.data::<AuthTokens>()?;

        if tokens.access_token.is_none() {
            return Err(Error::new("Unauthorized"));
        }

        Ok(())
    }
}
