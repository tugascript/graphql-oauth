use async_graphql::InputObject;

#[derive(InputObject)]
pub struct LoginInput {
    #[graphql(validator(email, min_length = 5, max_length = 200))]
    pub email: String,
    #[graphql(validator(min_length = 1))]
    pub password: String,
}
