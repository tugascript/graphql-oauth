use async_graphql::Union;

use crate::common::models::Message;

use super::AuthType;

#[derive(Union)]
pub enum LoginType {
    Message(Message),
    Auth(AuthType),
}
