use async_graphql::{CustomValidator, InputObject};

use crate::common::service::{error_handler, validate_jwt, validate_passwords};

#[derive(InputObject)]
pub struct ResetPasswordInput {
    pub token: String,
    pub password1: String,
    pub password2: String,
}

pub struct ResetPasswordValidator;

impl CustomValidator<ResetPasswordInput> for ResetPasswordValidator {
    fn check(&self, value: &ResetPasswordInput) -> Result<(), String> {
        let validations = [
            validate_jwt(&value.token),
            validate_passwords(&value.password1, &value.password2),
        ];
        error_handler(&validations)
    }
}
