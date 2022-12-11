use async_graphql::{Context, Object, Result};

use crate::common::models::Message;

use super::{
    dtos::{
        ChangeEmailInput, ChangePasswordInput, ChangePasswordValidator, LoginInput, RegisterInput,
        RegisterValidator, ResetPasswordInput, ResetPasswordValidator,
    },
    guards::AuthGuard,
    models::{AuthType, LoginType},
    service::{
        change_email, change_password, confirm_user, login_user, logout, refresh_access,
        register_user, reset_password, reset_password_email,
    },
};

#[derive(Default)]
pub struct AuthMutation;

#[Object]
impl AuthMutation {
    async fn register(
        &self,
        ctx: &Context<'_>,
        #[graphql(validator(custom = "RegisterValidator"))] input: RegisterInput,
    ) -> Result<Message> {
        register_user(ctx, input).await
    }

    async fn confirm_account(
        &self,
        ctx: &Context<'_>,
        #[graphql(validator(
            regex = r"^[A-Za-z0-9-_=]+\.[A-Za-z0-9-_=]+\.?[A-Za-z0-9-_.+/=]*$",
            min_length = 20
        ))]
        token: String,
    ) -> Result<AuthType> {
        confirm_user(ctx, token).await
    }

    async fn login(&self, ctx: &Context<'_>, input: LoginInput) -> Result<LoginType> {
        login_user(ctx, input).await
    }

    async fn reset_password_email(
        &self,
        ctx: &Context<'_>,
        #[graphql(validator(email, min_length = 5, max_length = 200))] email: String,
    ) -> Result<Message> {
        reset_password_email(ctx, email).await
    }

    async fn reset_password(
        &self,
        ctx: &Context<'_>,
        #[graphql(validator(custom = "ResetPasswordValidator"))] input: ResetPasswordInput,
    ) -> Result<Message> {
        reset_password(ctx, input).await
    }

    #[graphql(guard = "AuthGuard")]
    async fn change_password(
        &self,
        ctx: &Context<'_>,
        #[graphql(validator(custom = "ChangePasswordValidator"))] input: ChangePasswordInput,
    ) -> Result<AuthType> {
        change_password(ctx, input).await
    }

    #[graphql(guard = "AuthGuard")]
    async fn change_email(&self, ctx: &Context<'_>, input: ChangeEmailInput) -> Result<AuthType> {
        change_email(ctx, input).await
    }

    #[graphql(guard = "AuthGuard")]
    async fn logout(&self, ctx: &Context<'_>) -> Result<Message> {
        logout(ctx)
    }

    async fn refresh_access(&self, ctx: &Context<'_>) -> Result<AuthType> {
        refresh_access(ctx).await
    }
}
