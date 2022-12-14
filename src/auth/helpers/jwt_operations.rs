use async_graphql::{Error, Result};
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

use entities::user::Model;

#[derive(Debug, Serialize, Deserialize)]
pub struct EmailToken {
    pub id: i32,
    pub version: i16,
}

impl From<&Model> for EmailToken {
    fn from(model: &Model) -> Self {
        Self {
            id: model.id.to_owned(),
            version: model.version.to_owned(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    iss: String,
    sub: String,
    iat: i64,
    exp: i64,
    user: EmailToken,
}

impl Claims {
    fn create_token(
        user: &Model,
        secret: &str,
        exp: i64,
        iss: String,
        sub: String,
    ) -> Result<String> {
        let now = Utc::now();
        let claims = Claims {
            sub,
            iss,
            iat: now.timestamp(),
            exp: (now + Duration::seconds(exp)).timestamp(),
            user: EmailToken::from(user),
        };

        if let Ok(token) = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(secret.as_bytes()),
        ) {
            Ok(token)
        } else {
            Err(Error::new("Could not create token"))
        }
    }

    fn decode_token(secret: &str, token: &str) -> Result<EmailToken> {
        let claims = decode::<Claims>(
            token,
            &DecodingKey::from_secret(secret.as_bytes()),
            &Validation::default(),
        );

        match claims {
            Ok(s) => Ok(s.claims.user),
            _ => Err(Error::from("Invalid token")),
        }
    }
}

pub enum TokenType {
    Reset,
    Confirmation,
    Refresh,
}

pub fn create_token(
    token_type: TokenType,
    user: &Model,
    secret: &str,
    exp: i64,
    iss: &str,
) -> Result<String> {
    let sub = match token_type {
        TokenType::Reset => "reset".to_owned(),
        TokenType::Confirmation => "confirmation".to_owned(),
        TokenType::Refresh => "refresh".to_owned(),
    };

    Claims::create_token(user, secret, exp, iss.to_owned(), sub)
}

pub fn decode_token(token: &str, secret: &str) -> Result<EmailToken> {
    Claims::decode_token(secret, token)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AccessToken {
    pub id: i32,
}

impl From<&Model> for AccessToken {
    fn from(model: &Model) -> Self {
        Self {
            id: model.id.to_owned(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct AccessClaims {
    iss: String,
    sub: String,
    iat: i64,
    exp: i64,
    user: AccessToken,
}

impl AccessClaims {
    fn create_token(user: &Model, private_key: &str, exp: i64, iss: String) -> Result<String> {
        let now = Utc::now();
        let claims = AccessClaims {
            iss,
            sub: "access".to_owned(),
            iat: now.timestamp(),
            exp: (now + Duration::seconds(exp)).timestamp(),
            user: AccessToken::from(user),
        };
        let header = Header::new(Algorithm::RS256);
        let enconding_key = match EncodingKey::from_rsa_pem(private_key.as_bytes()) {
            Ok(key) => key,
            Err(_) => return Err(Error::from("Could not create token")),
        };

        if let Ok(token) = encode(&header, &claims, &enconding_key) {
            Ok(token)
        } else {
            Err(Error::new("Could not create token"))
        }
    }

    fn decode_token(public_key: &str, token: &str) -> Result<AccessToken> {
        let decoding_key = match DecodingKey::from_rsa_pem(public_key.as_bytes()) {
            Ok(key) => key,
            Err(_) => return Err(Error::from("Could not decode token")),
        };

        let claims =
            decode::<AccessClaims>(token, &decoding_key, &Validation::new(Algorithm::RS256));

        match claims {
            Ok(s) => Ok(s.claims.user),
            _ => Err(Error::from("Invalid token")),
        }
    }
}

pub fn create_access_token(user: &Model, private_key: &str, exp: i64, iss: &str) -> Result<String> {
    AccessClaims::create_token(user, private_key, exp, iss.to_owned())
}

pub fn decode_access_token(token: &str, public_key: &str) -> Result<AccessToken> {
    AccessClaims::decode_token(public_key, token)
}
