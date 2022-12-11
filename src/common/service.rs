use unicode_segmentation::UnicodeSegmentation;

use super::helpers::{
    multi_spaces_regex, new_line_regex,
    password_validator::password_validator,
    regexes::{email_regex, jwt_regex, name_regex},
};

pub fn format_name(name: &str) -> String {
    let mut title = name.trim().to_lowercase();
    title = new_line_regex().replace_all(&title, " ").to_string();
    title = multi_spaces_regex().replace_all(&title, " ").to_string();
    let mut c = title.chars();

    match c.next() {
        None => title,
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}

pub fn validate_email(email: &str) -> Result<(), String> {
    let len = email.graphemes(true).count();

    if len < 5 {
        return Err("Email needs to be at least 5 characters long".to_string());
    }

    if len > 200 {
        return Err("Email needs to be at most 200 characters long".to_string());
    }

    if !email_regex().is_match(email) {
        return Err("Invalid email".to_string());
    }

    Ok(())
}

pub fn validate_name(name: &str) -> Result<(), String> {
    let len = name.graphemes(true).count();

    if len < 3 || len > 50 {
        return Err("Name needs to be between 3 and 50 characters.".to_string());
    }

    if !name_regex().is_match(name) {
        return Err("Invalid name".to_string());
    }

    Ok(())
}

pub fn validate_passwords(password1: &str, password2: &str) -> Result<(), String> {
    if password1.is_empty() {
        return Err("Password is required".to_string());
    }

    if password2.is_empty() {
        return Err("Confirmation Password is required".to_string());
    }

    if password1 != password2 {
        return Err("Passwords do not match".to_string());
    }

    let len = password1.graphemes(true).count();

    if len < 8 || len > 40 {
        return Err("Password needs to be between 8 and 40 characters.".to_string());
    }

    if !password_validator(password1) {
        return Err("Password needs to have at least one lowercase letter, one uppercase letter, one number and one symbol.".to_string());
    }

    Ok(())
}

pub fn validate_jwt(jwt: &str) -> Result<(), String> {
    let len = jwt.chars().count();

    if len < 20 || len > 500 {
        return Err("JWT needs to be between 20 and 500 characters.".to_string());
    }

    if !jwt_regex().is_match(jwt) {
        return Err("Invalid JWT".to_string());
    }

    Ok(())
}

fn create_error_vec(validations: &[Result<(), String>]) -> Vec<&str> {
    let mut errors = Vec::<&str>::new();

    for error in validations {
        if let Err(e) = error {
            errors.push(e);
        }
    }

    errors
}

pub fn error_handler(validations: &[Result<(), String>]) -> Result<(), String> {
    let errors = create_error_vec(validations);
    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors.join("\n"))
    }
}
