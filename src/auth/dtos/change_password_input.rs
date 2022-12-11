use async_graphql::{CustomValidator, InputObject};

use crate::common::service::{error_handler, validate_passwords};

#[derive(InputObject)]
pub struct ChangePasswordInput {
    pub old_password: String,
    pub password1: String,
    pub password2: String,
}

pub struct ChangePasswordValidator;

impl CustomValidator<ChangePasswordInput> for ChangePasswordValidator {
    fn check(&self, value: &ChangePasswordInput) -> Result<(), String> {
        let validations = [validate_passwords(&value.password1, &value.password2)];
        error_handler(&validations)
    }
}
