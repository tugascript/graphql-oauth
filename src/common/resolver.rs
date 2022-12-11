use async_graphql::{Object, Result};

#[derive(Default)]
pub struct CommonQuery;

#[Object]
impl CommonQuery {
    async fn health_check(&self) -> Result<String> {
        Ok("Ok".to_string())
    }
}
