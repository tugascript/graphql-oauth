use async_graphql::SimpleObject;
use serde::{Deserialize, Serialize};

use crate::users::models::User;

#[derive(SimpleObject, Serialize, Deserialize)]
pub struct AuthType {
    pub user: User,
    pub access_token: String,
}

impl AuthType {
    pub fn new(access_token: String, user: User) -> Self {
        Self { user, access_token }
    }
}
