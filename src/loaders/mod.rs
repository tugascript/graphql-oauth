use std::collections::HashMap;

use async_graphql::dataloader::*;
use async_graphql::*;

pub mod users_loader;

use crate::{config::Database, users::models::User};
use users_loader::*;

pub struct SeaOrmLoader {
    db: Database,
}

impl SeaOrmLoader {
    pub fn new(db: &Database) -> Self {
        Self { db: db.clone() }
    }
}

#[async_trait::async_trait]
impl Loader<UserId> for SeaOrmLoader {
    type Value = User;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[UserId]) -> Result<HashMap<UserId, Self::Value>, Self::Error> {
        load_users(self.db.get_connection(), keys).await
    }
}
