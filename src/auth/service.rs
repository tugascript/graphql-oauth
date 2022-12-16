use actix_web::{
    cookie::{time::Duration, Cookie},
    http::header::SET_COOKIE,
};
use async_graphql::{Context, Error, Result};
use generate_two_factor_code::verify_two_factor_code;
use redis::AsyncCommands;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, Condition, EntityTrait, PaginatorTrait, QueryFilter, Set,
};

use entities::user;

use crate::{
    common::{helpers::get_access_user::get_access_user, models::Message, service::format_name},
    config::{Cache, Database, Jwt, Mailer},
    gql_set_up::{AuthTokens, Environment},
    users::models::User,
};

use super::{
    dtos::{
        ChangeEmailInput, ChangePasswordInput, ConfirmLoginInput, LoginInput, RegisterInput,
        ResetPasswordInput,
    },
    helpers::{
        create_auth_tokens, create_token, decode_token, generate_two_factor_code, hash_password,
        send_confirmation_email, verify_password, TokenType,
    },
    models::{AuthType, LoginType},
};

/**
Register User (GraphQL Mutation)

Takes a validated register input and creates a new user, then sends a confirmation email.
 */
pub async fn register_user(ctx: &Context<'_>, input: RegisterInput) -> Result<Message> {
    let db = ctx.data::<Database>()?;
    let email = input.email.to_lowercase();
    let email_count = user::Entity::find()
        .filter(user::Column::Email.eq(email.to_owned()))
        .count(db.get_connection())
        .await?;

    if email_count > 0 {
        return Err(Error::from("Email already exists"));
    }

    let first_name = format_name(&input.first_name);
    let last_name = format_name(&input.last_name);
    let password_hash = hash_password(&input.password1)?;
    let user = user::ActiveModel {
        email: Set(email),
        first_name: Set(first_name),
        last_name: Set(last_name),
        password: Set(password_hash),
        ..Default::default()
    };
    let user = user.insert(db.get_connection()).await?;
    let jwt = ctx.data::<Jwt>()?;

    if let Some(code) = send_confirmation_email(ctx, &jwt, &user).await? {
        return Ok(Message::new(&code));
    }

    Ok(Message::new("User registered successfully"))
}

/**
Confirm User (GraphQL Mutation)

Takes the confirmation JWT and confirms the user.
 */
pub async fn confirm_user(ctx: &Context<'_>, token: String) -> Result<AuthType> {
    let jwt = ctx.data::<Jwt>()?;
    let user = decode_token(&token, &jwt.confirmation.secret)?;
    let db = ctx.data::<Database>()?;
    let user = user::Entity::find_by_id(user.id)
        .one(db.get_connection())
        .await?
        .ok_or(Error::from("User not found"))?;

    if user.confirmed {
        return Err(Error::from("User already confirmed"));
    }

    let mut user: user::ActiveModel = user.into();
    user.confirmed = Set(true);
    let user = user.update(db.get_connection()).await?;

    Ok(AuthType::new(
        create_auth_tokens(ctx, jwt, &user)?,
        User::from(user),
    ))
}

/**
Login User (GraphQL Mutation)

Takes a validated login input and if the user has two factor active sends a new login code to his email.
If not, creates a new auth tokens, saves the refresh-token in a http-only cookie and sends the access token
back to the front-end.
 */
pub async fn login_user(ctx: &Context<'_>, input: LoginInput) -> Result<LoginType> {
    let db = ctx.data::<Database>()?;
    let user = user::Entity::find()
        .filter(user::Column::Email.eq(input.email.to_lowercase()))
        .one(db.get_connection())
        .await?
        .ok_or(Error::from("Invalid credentials"))?;

    verify_password(&input.password, &user.password)?;
    let jwt = ctx.data::<Jwt>()?;

    if !user.confirmed {
        send_confirmation_email(ctx, &jwt, &user).await?;
        return Err(Error::from("User not confirmed"));
    }
    if user.two_factor_enabled {
        let (code, hash) = generate_two_factor_code()?;
        let mut cache_connection = ctx.data::<Cache>()?.get_connection().await?;
        cache_connection
            .set_ex(format!("2F_{}", user.id.to_string()), hash, 900)
            .await?;

        match ctx.data::<Environment>()? {
            Environment::Development => return Ok(LoginType::Message(Message::new(&code))),
            Environment::Production => {
                ctx.data::<Mailer>()?
                    .send_access_email(&user.email, &user.get_full_name(), &code)
                    .await?;
                return Ok(LoginType::Message(Message::new(
                    "Login code sent to your email",
                )));
            }
        }
    }

    Ok(LoginType::Auth(AuthType::new(
        create_auth_tokens(ctx, jwt, &user)?,
        User::from(user),
    )))
}

/**
 Confirm Login (GraphQL Mutation)

 Takes the login code and if it matches the one in the cache, creates a new auth tokens and
 sends the access token to the front-end.
*/
pub async fn confirm_login(ctx: &Context<'_>, input: ConfirmLoginInput) -> Result<AuthType> {
    let db = ctx.data::<Database>()?;
    let user = user::Entity::find()
        .filter(user::Column::Email.eq(input.email.to_lowercase()))
        .one(db.get_connection())
        .await?
        .ok_or(Error::from("Invalid credentials"))?;

    let mut cache_con = ctx.data::<Cache>()?.get_connection().await?;
    let code: Option<String> = cache_con.get(format!("2F_{}", user.id.to_string())).await?;

    if let Some(hashed_code) = code {
        verify_two_factor_code(&input.code, &hashed_code)?;
    } else {
        return Err(Error::from("Code has expired"));
    }

    let jwt = ctx.data::<Jwt>()?;
    Ok(AuthType::new(
        create_auth_tokens(ctx, jwt, &user)?,
        User::from(user),
    ))
}

/**
Reset Password Email (GraphQL Mutation)

Sends a reset password email to a given email if a user is associated with that email.
 */
pub async fn reset_password_email(ctx: &Context<'_>, email: String) -> Result<Message> {
    let db = ctx.data::<Database>()?;
    let user = match user::Entity::find()
        .filter(user::Column::Email.eq(email.to_lowercase()))
        .one(db.get_connection())
        .await?
    {
        Some(user) => user,
        None => return Ok(Message::new("Reset password email sent")),
    };

    let jwt = ctx.data::<Jwt>()?;
    let token = create_token(
        TokenType::Reset,
        &user,
        &jwt.reset.secret,
        jwt.reset.exp,
        &jwt.api_id,
    )?;

    match ctx.data::<Environment>()? {
        Environment::Development => return Ok(Message::new(&token)),
        Environment::Production => {
            ctx.data::<Mailer>()?
                .send_password_reset_email(&user.email, &user.get_full_name(), &token)
                .await?;

            return Ok(Message::new("Reset password email sent"));
        }
    }
}

/**
Reset Password (GraphQL Mutation)

Takes a reset password token and a new password input and if the token is valid, updates the user's password.
 */
pub async fn reset_password(ctx: &Context<'_>, input: ResetPasswordInput) -> Result<Message> {
    let jwt = ctx.data::<Jwt>()?;
    let user = decode_token(&input.token, &jwt.reset.secret)?;
    let db = ctx.data::<Database>()?;
    let user = user::Entity::find()
        .filter(
            Condition::all()
                .add(user::Column::Id.eq(user.id))
                .add(user::Column::Version.eq(user.version)),
        )
        .one(db.get_connection())
        .await?
        .ok_or(Error::from("Token is invalid"))?;

    if input.password1 != input.password2 {
        return Err(Error::from("Passwords do not match"));
    }

    let new_version = user.version + 1;
    let mut user: user::ActiveModel = user.into();
    user.password = Set(hash_password(&input.password1)?);
    user.version = Set(new_version);
    user.update(db.get_connection()).await?;

    Ok(Message::new("Password reset successfully"))
}

/**
Change Password (GraphQL Mutation)

Takes a current password and a new password input and if the current password is valid, updates the user's password.
On updating the password, the user version is incremented so all old logins are logged out.
 */
pub async fn change_password(ctx: &Context<'_>, input: ChangePasswordInput) -> Result<AuthType> {
    let user = get_access_user(ctx)?;
    let db = ctx.data::<Database>()?;
    let user = user::Entity::find_by_id(user.id)
        .one(db.get_connection())
        .await?
        .ok_or(Error::from("User not found"))?;
    verify_password(&input.old_password, &user.password)?;
    let new_version = user.version + 1;
    let mut user: user::ActiveModel = user.into();
    user.password = Set(hash_password(&input.password1)?);
    user.version = Set(new_version);
    let user = user.update(db.get_connection()).await?;
    let jwt = ctx.data::<Jwt>()?;

    Ok(AuthType::new(
        create_auth_tokens(ctx, jwt, &user)?,
        User::from(user),
    ))
}

/**
Change Email (GraphQL Mutation)

Takes a current password and a new email input and if the current password is valid, updates the user's email.
On updating the email, the user version is incremented so all old logins are logged out.
 */
pub async fn change_email(ctx: &Context<'_>, input: ChangeEmailInput) -> Result<AuthType> {
    let email = input.new_email.to_lowercase();
    let db = ctx.data::<Database>()?;
    let user_count = user::Entity::find()
        .filter(user::Column::Email.eq(email.to_owned()))
        .count(db.get_connection())
        .await?;

    if user_count > 0 {
        return Err(Error::from("Email already in use"));
    }

    let user = get_access_user(ctx)?;
    let user = user::Entity::find_by_id(user.id)
        .one(db.get_connection())
        .await?
        .ok_or(Error::from("User not found"))?;
    let new_version = user.version + 1;
    let mut user: user::ActiveModel = user.into();
    user.email = Set(email);
    user.version = Set(new_version);
    let user = user.update(db.get_connection()).await?;
    let jwt = ctx.data::<Jwt>()?;

    Ok(AuthType::new(
        create_auth_tokens(ctx, jwt, &user)?,
        User::from(user),
    ))
}

fn remove_refresh_cookie(ctx: &Context<'_>, jwt: &Jwt) {
    let mut cookie = Cookie::build(jwt.refresh_cookie.to_owned(), "".to_owned())
        .path("/api/graphql")
        .max_age(Duration::seconds(jwt.refresh.exp))
        .http_only(true)
        .finish();
    cookie.make_removal();
    ctx.insert_http_header(SET_COOKIE, cookie.to_string());
}

/**
Log Out (GraphQL Mutation)

Invalidates the refresh token, so the user becomes log out.
 */
pub fn logout(ctx: &Context<'_>) -> Result<Message> {
    let jwt = ctx.data::<Jwt>()?;
    remove_refresh_cookie(ctx, jwt);
    Ok(Message::new("Logged out successfully"))
}

/**
Refresh Access (GraphQL Mutation)

Takes a refresh token and if the token is valid, returns a new access token inside an AuthType.
 */
pub async fn refresh_access(ctx: &Context<'_>) -> Result<AuthType> {
    let jwt = ctx.data::<Jwt>()?;
    let tokens = ctx.data::<AuthTokens>()?;
    let refresh_token = tokens
        .refresh_token
        .as_ref()
        .ok_or(Error::from("Unauthorized"))?;
    let auth_user = decode_token(refresh_token, &jwt.refresh.secret)?;
    let db = ctx.data::<Database>()?;
    let user = user::Entity::find()
        .filter(
            Condition::all()
                .add(user::Column::Id.eq(auth_user.id))
                .add(user::Column::Version.eq(auth_user.version)),
        )
        .one(db.get_connection())
        .await?;

    if let Some(user) = user {
        Ok(AuthType::new(
            create_auth_tokens(ctx, jwt, &user)?,
            User::from(user),
        ))
    } else {
        remove_refresh_cookie(ctx, jwt);
        Err(Error::from("Unauthorized"))
    }
}
