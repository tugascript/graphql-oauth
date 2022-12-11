use async_graphql::{Error, Result};
use bcrypt::{hash, verify};
use rand::Rng;

fn generate_code() -> String {
    const NUMERIC_SET: &[u8] = b"0123456789";
    const CODE_LEN: usize = 6;
    let mut rng = rand::thread_rng();
    (0..CODE_LEN)
        .map(|_| {
            let idx = rng.gen_range(0..NUMERIC_SET.len());
            NUMERIC_SET[idx] as char
        })
        .collect::<String>()
}

pub fn generate_two_factor_code() -> Result<(String, String)> {
    let code = generate_code();

    if let Ok(hash) = hash(&code, 5) {
        return Ok((code, hash));
    }

    Err(Error::new("Error generating two factor code"))
}

pub fn verify_two_factor_code(code: &str, hashed_code: &str) -> Result<()> {
    if let Ok(is_valid) = verify(code, hashed_code) {
        if is_valid {
            return Ok(());
        } else {
            return Err(Error::new("Invalid two factor code"));
        }
    }

    Err(Error::new("Error verifying two factor code"))
}
