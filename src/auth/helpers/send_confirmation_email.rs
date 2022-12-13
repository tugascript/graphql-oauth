use crate::{
    config::{Jwt, Mailer},
    gql_set_up::Environment,
};

use super::{create_token, TokenType};
use async_graphql::{Context, Result};
use entities::user::Model;

pub async fn send_confirmation_email(
    ctx: &Context<'_>,
    jwt: &Jwt,
    user: &Model,
) -> Result<Option<String>> {
    let confirmation_token = create_token(
        TokenType::Confirmation,
        user,
        &jwt.confirmation.secret,
        jwt.confirmation.exp,
        &jwt.api_id,
    )?;

    if ctx.data::<Environment>()?.0 == "production" {
        ctx.data::<Mailer>()?
            .send_confirmation_email(&user.email, &user.get_full_name(), &confirmation_token)
            .await?;

        return Ok(None);
    }

    Ok(Some(confirmation_token))
}
