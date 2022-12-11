use async_graphql::{Error, Result};
use entities::user::{Column, Entity};
use std::collections::HashMap;

use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};

use crate::users::models::User;

#[derive(Hash, Eq, PartialEq, Clone, Debug)]
pub struct UserId(pub i32);

pub async fn load_users(
    connection: &DatabaseConnection,
    keys: &[UserId],
) -> Result<HashMap<UserId, User>> {
    let mut users_hash: HashMap<UserId, User> = HashMap::new();
    let users = Entity::find()
        .filter(Column::Id.is_in(keys.iter().map(|k| k.0).collect::<Vec<i32>>()))
        .all(connection)
        .await?;

    if users.len() != keys.len() {
        return Err(Error::from("User not found"));
    }

    for user in users {
        users_hash.insert(UserId(user.id), User::from(user));
    }

    Ok(users_hash)
}
