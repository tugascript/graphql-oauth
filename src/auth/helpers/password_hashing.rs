use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
    Argon2, PasswordHash, PasswordVerifier,
};

pub fn hash_password(password: &str) -> Result<String, String> {
    let salt = SaltString::generate(&mut OsRng);
    let hash = Argon2::default().hash_password(password.as_bytes(), &salt);

    if hash.is_err() {
        return Err("Could not hash password, please try again".to_owned());
    }

    Ok(hash.unwrap().to_string())
}

pub fn verify_password(password: &str, str_hash: &str) -> Result<(), String> {
    let hash = PasswordHash::new(&str_hash).map_err(|e| e.to_string())?;
    Ok(Argon2::default()
        .verify_password(password.as_bytes(), &hash)
        .map_err(|_| "Invalid credentials".to_owned())?)
}
