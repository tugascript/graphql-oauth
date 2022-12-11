use async_graphql::{Context, Error, Result};
use sea_orm::{EntityTrait, ModelTrait};

use entities::user::{Entity, Model};

use crate::{
    auth::{helpers::verify_password, service::logout},
    common::{helpers::get_access_user, models::Message},
    config::Database,
};

pub async fn user_by_id(db: &Database, id: i32) -> Result<Model> {
    Entity::find_by_id(id)
        .one(db.get_connection())
        .await?
        .ok_or(Error::new("User not found"))
}

pub async fn delete_account(ctx: &Context<'_>, password: String) -> Result<Message> {
    let user = get_access_user(ctx)?;
    let db = ctx.data::<Database>()?;
    let user = user_by_id(db, user.id).await?;
    verify_password(&password, &user.password)?;
    let res = user.delete(db.get_connection()).await?;

    if res.rows_affected == 0 {
        return Err(Error::new("Failed to delete account"));
    }

    logout(ctx)?;
    Ok(Message::new("Account deleted successfully"))
}
