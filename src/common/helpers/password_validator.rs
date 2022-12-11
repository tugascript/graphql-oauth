#[derive(Default)]
struct PasswordValidity {
    has_lowercase: bool,
    has_uppercase: bool,
    has_number: bool,
    has_symbol: bool,
}

impl PasswordValidity {
    fn new() -> Self {
        Self::default()
    }
}

pub fn password_validator(password: &str) -> bool {
    let mut validity = PasswordValidity::new();

    for char in password.chars() {
        if char.is_lowercase() {
            validity.has_lowercase = true;
        } else if char.is_uppercase() {
            validity.has_uppercase = true;
        } else if char.is_numeric() {
            validity.has_number = true;
        } else {
            validity.has_symbol = true;
        }
    }

    let mut passed: u16 = 0;

    if validity.has_number {
        passed += 1;
    }
    if validity.has_lowercase {
        passed += 1;
    }
    if validity.has_uppercase {
        passed += 1;
    }
    if validity.has_symbol {
        passed += 1;
    }

    return passed * 100 / 4 == 100;
}
