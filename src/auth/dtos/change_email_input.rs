use async_graphql::InputObject;

#[derive(InputObject)]
pub struct ChangeEmailInput {
    #[graphql(validator(email, min_length = 5, max_length = 200))]
    pub new_email: String,
    #[graphql(validator(min_length = 1))]
    pub password: String,
}
