use async_graphql::{ComplexObject, SimpleObject, ID};
use entities::user::Model;
use sea_orm::entity::prelude::DateTime;
use serde::{Deserialize, Serialize};

#[derive(SimpleObject, Serialize, Deserialize, Clone)]
#[graphql(complex)]
pub struct User {
    pub id: ID,
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    #[graphql(skip)]
    pub created_at: DateTime,
    #[graphql(skip)]
    pub updated_at: DateTime,
}

impl From<Model> for User {
    fn from(model: Model) -> Self {
        Self {
            id: ID(model.id.to_string()),
            email: model.email,
            first_name: model.first_name,
            last_name: model.last_name,
            created_at: model.created_at,
            updated_at: model.updated_at,
        }
    }
}

#[ComplexObject]
impl User {
    async fn create_timestamp(&self) -> i64 {
        self.created_at.timestamp()
    }

    async fn updated_timestamp(&self) -> i64 {
        self.updated_at.timestamp()
    }
}
