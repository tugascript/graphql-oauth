use async_graphql::InputObject;

#[derive(InputObject)]
pub struct ConfirmLoginInput {
    #[graphql(validator(email, min_length = 5, max_length = 200))]
    pub email: String,
    #[graphql(validator(min_length = 6, max_length = 6, regex = r"^[0-9]+$"))]
    pub code: String,
}
