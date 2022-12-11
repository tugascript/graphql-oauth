use async_graphql::{CustomValidator, InputObject};

use crate::common::service::{error_handler, validate_email, validate_name, validate_passwords};

#[derive(InputObject)]
pub struct RegisterInput {
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub password1: String,
    pub password2: String,
}

pub struct RegisterValidator;

impl CustomValidator<RegisterInput> for RegisterValidator {
    fn check(&self, value: &RegisterInput) -> Result<(), String> {
        let validations = [
            validate_email(&value.email),
            validate_name(&value.first_name),
            validate_name(&value.last_name),
            validate_passwords(&value.password1, &value.password2),
        ];
        error_handler(&validations)
    }
}
