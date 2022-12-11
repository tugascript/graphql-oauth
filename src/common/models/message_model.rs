use async_graphql::SimpleObject;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(SimpleObject, Serialize, Deserialize)]
pub struct Message {
    pub id: String,
    pub message: String,
}

impl Message {
    pub fn new(message: &str) -> Self {
        let id = Uuid::new_v4().to_string();

        Self {
            id,
            message: message.to_string(),
        }
    }
}
