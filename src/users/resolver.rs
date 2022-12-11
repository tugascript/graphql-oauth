use async_graphql::{dataloader::DataLoader, Context, Error, Object, Result};

use super::{
    models::User,
    service::{delete_account, user_by_id},
};
use crate::{
    auth::guards::AuthGuard,
    common::{helpers::get_access_user, models::Message},
    config::Database,
    loaders::{users_loader::UserId, SeaOrmLoader},
};

#[derive(Default)]
pub struct UsersQuery;

#[Object]
impl UsersQuery {
    #[graphql(guard = "AuthGuard")]
    async fn me(&self, ctx: &Context<'_>) -> Result<User> {
        let user = get_access_user(ctx)?;
        let db = ctx.data::<Database>()?;
        let user = user_by_id(db, user.id).await?;
        Ok(User::from(user))
    }

    #[graphql(entity)]
    async fn find_user_by_id(
        &self,
        ctx: &Context<'_>,
        #[graphql(validator(minimum = 1))] id: i32,
    ) -> Result<User> {
        ctx.data::<DataLoader<SeaOrmLoader>>()?
            .load_one(UserId(id))
            .await?
            .ok_or(Error::from("Not found"))
    }
}

#[derive(Default)]
pub struct UsersMutation;

#[Object]
impl UsersMutation {
    #[graphql(guard = "AuthGuard")]
    async fn delete_account(
        &self,
        ctx: &Context<'_>,
        #[graphql(validator(min_length = 1))] password: String,
    ) -> Result<Message> {
        delete_account(ctx, password).await
    }
}
